#[cfg(feature = "async")]
mod async_abci;
#[cfg(feature = "async")]
pub use async_abci::{dispatch, Application};

#[cfg(feature = "sync")]
mod sync_abci;
#[cfg(feature = "sync")]
pub use sync_abci::{sync_dispatch, SyncApplication};
