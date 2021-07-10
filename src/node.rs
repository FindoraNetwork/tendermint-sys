//! Tendermint Node.
//!
//! Create, start or stop tendermint node.

use crate::raw::{new_node, NodeIndex};
use crate::{Error, Result};
use tendermint::config::TendermintConfig;
use tendermint_abci::Application as SyncApplication;

pub struct Node<A: SyncApplication> {
    index: NodeIndex,
    app: A,
}

impl<A: SyncApplication> Node<A> {
    pub fn new(config: TendermintConfig, app: A) -> Result<Self> {
        // let index = unsafe {};
        Ok(Node { index: 0, app })
    }
}
