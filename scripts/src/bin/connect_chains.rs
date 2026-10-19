use cw_orch::prelude::{networks::*, *};
use cw_orch_interchain::prelude::*;
use cw_orch_polytone::Polytone;
/// This stays unsued and is used for reference for channel creation
pub const POLYTONE_VERSION: &str = "polytone-1";
fn main() {
    let src_chain = XION_TESTNET_1;
    let dst_chain = PION_1;

    instantiate_two_chains(src_chain, dst_chain).unwrap();
}

/// Instantiates a polytone connection between two chains
/// The channel needs to be created by a relayer.
/// It leverages the deployment id of abstract to have multiple connections on the same chain
fn instantiate_two_chains(src_chain: ChainInfo, dst_chain: ChainInfo) -> anyhow::Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::init();

    let src_daemon = DaemonBuilder::new(src_chain).build()?;
    let src_polytone = Polytone::load_from(src_daemon.clone())?;

    let dst_daemon = DaemonBuilder::new(dst_chain)
        .state(src_daemon.state())
        .build()?;
    let dst_polytone = Polytone::load_from(dst_daemon.clone())?;

    let interchain =
        DaemonInterchain::from_daemons(vec![src_daemon, dst_daemon], &ChannelCreationValidator);

    src_polytone.connect(&dst_polytone, &interchain)?;

    Ok(())
}
