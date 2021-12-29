pub mod raw;

mod node;
pub use node::Node;

mod error;
pub use error::{Error, Result};

mod init;
pub use init::init_home;

pub enum NodeEnum {
    FullNode,
    Validator,
    Seed,
}

impl NodeEnum {
    pub fn code(&self) -> raw::NodeType {
        match self {
            NodeEnum::FullNode => 0,
            NodeEnum::Validator => 1,
            NodeEnum::Seed => 2,
        }
    }
}
