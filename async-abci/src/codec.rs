use bytes::{Buf, BufMut, BytesMut};
use prost::Message;
use std::marker::PhantomData;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, AsyncRead, AsyncWrite},
};

use crate::error::Error;

/// The maximum number of bytes we expect in a varint. We use this to check if
/// we're encountering a decoding error for a varint.
pub const MAX_VARINT_LENGTH: usize = 16;

pub struct ICodec<R, I> {
    stream: R,
    // Long-running read buffer
    read_buf: BytesMut,
    // Fixed-length read window
    read_window: Vec<u8>,
    _incoming: PhantomData<I>,
}

impl<R, I> ICodec<R, I>
where
    I: Message + Default,
{
    /// Constructor.
    pub fn new(stream: R, read_buf_size: usize) -> Self {
        Self {
            stream,
            read_buf: BytesMut::new(),
            read_window: vec![0_u8; read_buf_size],
            _incoming: Default::default(),
        }
    }
}

// Iterating over a codec produces instances of `Result<I>`.
impl<R, I> ICodec<R, I>
where
    R: AsyncRead + Unpin,
    I: Message + Default,
{
    pub async fn next(&mut self) -> Option<Result<I, Error>> {
        loop {
            // Try to decode an incoming message from our buffer first
            match decode_length_delimited::<I>(&mut self.read_buf) {
                Ok(Some(incoming)) => return Some(Ok(incoming)),
                Err(e) => return Some(Err(e)),
                _ => (), // not enough data to decode a message, let's continue.
            }

            // If we don't have enough data to decode a message, try to read
            // more
            let bytes_read = match self.stream.read(self.read_window.as_mut()).await {
                Ok(br) => br,
                Err(e) => return Some(Err(Error::StdIoError(e))),
            };
            if bytes_read == 0 {
                // The underlying stream terminated
                return None;
            }
            self.read_buf
                .extend_from_slice(&self.read_window[..bytes_read]);
        }
    }
}

pub struct OCodec<W, O> {
    stream: W,
    write_buf: BytesMut,
    _incoming: PhantomData<O>,
}

impl<W, O> OCodec<W, O>
where
    O: Message + Default,
{
    /// Constructor.
    pub fn new(stream: W) -> Self {
        Self {
            stream,
            write_buf: BytesMut::default(),
            _incoming: Default::default(),
        }
    }
}

impl<W, O> OCodec<W, O>
where
    W: AsyncWrite + Unpin,
    O: Message,
{
    /// Send a message using this codec.
    pub async fn send(&mut self, message: O) -> Result<(), Error> {
        encode_length_delimited(message, &mut self.write_buf)?;
        while !self.write_buf.is_empty() {
            let bytes_written = self
                .stream
                .write(self.write_buf.as_ref())
                .await
                .map_err(Error::StdIoError)?;

            if bytes_written == 0 {
                return Err(Error::StdIoError(std::io::Error::new(
                    std::io::ErrorKind::WriteZero,
                    "failed to write to underlying stream",
                )));
            }
            self.write_buf.advance(bytes_written);
        }

        self.stream.flush().await.map_err(Error::StdIoError)?;

        Ok(())
    }
}

/// Encode the given message with a length prefix.
pub fn encode_length_delimited<M, B>(message: M, mut dst: &mut B) -> Result<(), Error>
where
    M: Message,
    B: BufMut,
{
    let mut buf = BytesMut::new();
    message.encode(&mut buf).map_err(Error::ProstEncodeError)?;

    let buf = buf.freeze();
    prost::encoding::encode_varint(buf.len() as u64, &mut dst);
    dst.put(buf);
    Ok(())
}

/// Attempt to decode a message of type `M` from the given source buffer.
pub fn decode_length_delimited<M>(src: &mut BytesMut) -> Result<Option<M>, Error>
where
    M: Message + Default,
{
    let src_len = src.len();
    let mut tmp = src.clone().freeze();
    let encoded_len = match prost::encoding::decode_varint(&mut tmp) {
        Ok(len) => len,
        // We've potentially only received a partial length delimiter
        Err(_) if src_len <= MAX_VARINT_LENGTH => return Ok(None),
        Err(e) => return Err(Error::ProstDecodeError(e)),
    };
    let remaining = tmp.remaining() as u64;
    if remaining < encoded_len {
        // We don't have enough data yet to decode the entire message
        Ok(None)
    } else {
        let delim_len = src_len - tmp.remaining();
        // We only advance the source buffer once we're sure we have enough
        // data to try to decode the result.
        src.advance(delim_len + (encoded_len as usize));

        let mut result_bytes = BytesMut::from(tmp.split_to(encoded_len as usize).as_ref());
        let res = M::decode(&mut result_bytes).map_err(Error::ProstDecodeError)?;

        Ok(Some(res))
    }
}
