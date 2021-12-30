pub mod raw;

mod node;
pub use node::Node;

mod error;
pub use error::{Error, Result};

mod init;
pub use init::init_home;

pub enum NodeType {
    FullNode,
    Validator,
    Seed,
}

impl NodeType {
    pub fn code(&self) -> raw::NodeType {
        match self {
            NodeType::FullNode => 0,
            NodeType::Validator => 1,
            NodeType::Seed => 2,
        }
    }
}
