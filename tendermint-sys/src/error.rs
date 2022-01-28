use std::io;

#[derive(Debug)]
pub enum Error {
    ConfigParseFailed,
    LoadNodeKeyFailed,
    CreatNodeFailed,
    GoLogLevelError,
    NoNodeIndex,
    Unknown,
    GenesisFileSaveError,
    StdFsError(io::Error),
    GetPublicKeyError,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::StdFsError(e)
    }
}

impl Error {
    pub fn from_new_node_error(code: i32) -> Self {
        match code {
            -1 => Error::ConfigParseFailed,
            -2 => Error::LoadNodeKeyFailed,
            -3 => Error::CreatNodeFailed,
            -4 => Error::GoLogLevelError,
            _ => Error::Unknown,
        }
    }

    pub fn from_new_config_error(code: i32) -> Self {
        match code {
            -1 => Error::GoLogLevelError,
            -2 => Error::LoadNodeKeyFailed,
            -3 => Error::GetPublicKeyError,
            -4 => Error::GenesisFileSaveError,
            _ => Error::Unknown,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
