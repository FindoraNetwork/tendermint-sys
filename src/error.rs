pub enum Error {
    ConfigParseFailed,
    LoadNodeKeyFailed,
    CreatNodeFailed,
    NoNodeIndex,
    Unknown,
}

impl Error {
    pub fn from_new_node_error(code: i32) -> Self {
        match code {
            -1 => Error::ConfigParseFailed,
            -2 => Error::LoadNodeKeyFailed,
            -3 => Error::CreatNodeFailed,
            _ => Error::Unknown,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
