pub mod accounts;
pub mod ack;
pub mod callbacks;
pub mod ibc;

pub mod erc20;
pub mod evm;
pub mod handshake;

// Codegen bindings
#[rustfmt::skip]
#[cfg(feature = "bind")]
pub mod bind;

pub const EVM_NOTE_ID: &str = "abstract:evm-note";
pub const POLYTONE_EVM_VERSION: &str = env!("CARGO_PKG_VERSION");
