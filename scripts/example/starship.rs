use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
use cosmos_sdk_proto::ibc::applications::transfer::v1::MsgTransfer;
use cosmos_sdk_proto::ibc::applications::transfer::v1::MsgTransferResponse;
use cosmos_sdk_proto::traits::MessageExt;
use cosmwasm_std::IbcOrder;
use cw_orch::environment::ChainState;
use cw_orch::prelude::*;
use cw_orch::tokio::runtime::Runtime;
use cw_orch_interchain::{prelude::*, Starship};
use cw_orch_polytone::PolytoneConnection;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
fn main() -> cw_orch::anyhow::Result<()> {
    pretty_env_logger::init();
    let rt = Runtime::new()?;
    let starship = Starship::new(rt.handle(), None)?;
    let interchain = starship.interchain_env();
    let src_chain = interchain.chain("juno-1")?;
    let dst_chain = interchain.chain("stargaze-1")?;

    let src_chain_id = src_chain.chain_id();
    let dst_chain_id = dst_chain.chain_id();

    // We get the polytone connection
    let polytone_connection =
        PolytoneConnection::deploy_between_if_needed(&interchain, &src_chain_id, &dst_chain_id)?;

    // We send an empty message on the note side
    let tx_response = polytone_connection.send_message(vec![])?;

    interchain.check_ibc(&src_chain_id, tx_response)?;

    Ok(())
}
