#[derive(Debug)]
pub enum Error {
    ConfigParseFailed,
    LoadNodeKeyFailed,
    CreatNodeFailed,
    GoLogLevelError,
    NoNodeIndex,
    Unknown,
}

impl Error {
    pub fn from_new_node_error(code: i32) -> Self {
        println!("{}", code);
        match code {
            -1 => Error::ConfigParseFailed,
            -2 => Error::LoadNodeKeyFailed,
            -3 => Error::CreatNodeFailed,
            -4 => Error::GoLogLevelError,
            _ => Error::Unknown,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
