use cw_orch::environment::ChainState;
use cw_orch::{prelude::networks::*, prelude::*};
use cw_orch_interchain::prelude::*;
use cw_orch_polytone::interchain::PolytoneConnection;
fn main() {
    let src_chain = XION_TESTNET_1;
    let dst_chain = PION_1;
    verify_deployment(src_chain, dst_chain).unwrap();
}

fn verify_deployment(src_chain: ChainInfo, dst_chain: ChainInfo) -> anyhow::Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::init();

    let src_daemon = DaemonBuilder::new(src_chain.clone()).build()?;
    let dst_daemon = DaemonBuilder::new(dst_chain.clone())
        .state(src_daemon.state())
        .build()?;

    let interchain = DaemonInterchain::from_daemons(
        vec![src_daemon.clone(), dst_daemon.clone()],
        &ChannelCreationValidator,
    );

    let polytone_connection = PolytoneConnection::load_from(src_daemon, dst_daemon);

    // We send an empty message on the note side
    let tx_response = polytone_connection.send_message(vec![])?;

    interchain.await_and_check_packets(src_chain.chain_id, tx_response)?;

    Ok(())
}
