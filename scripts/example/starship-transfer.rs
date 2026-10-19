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
    let token = src_chain.state().chain_data.gas_denom.clone();

    let src_chain_id = src_chain.chain_id();
    let dst_chain_id = dst_chain.chain_id();

    let channel = interchain.create_channel(
        &src_chain_id,
        &dst_chain_id,
        &PortId::transfer(),
        &PortId::transfer(),
        "ics20-1",
        Some(IbcOrder::Unordered),
    )?;

    // We send an empty message on the note side
    let tx_response = src_chain.commit_any::<MsgTransferResponse>(
        vec![MsgTransfer {
            source_port: "transfer".to_string(),
            source_channel: channel
                .interchain_channel
                .get_chain(&src_chain_id)?
                .channel
                .unwrap()
                .to_string(),
            token: Some(Coin {
                denom: token,
                amount: "10".to_string(),
            }),
            sender: src_chain.sender().to_string(),
            receiver: dst_chain.sender().to_string(),
            timeout_height: None,
            timeout_timestamp: 1_818_617_994_000_000_000,
        }
        .to_any()?],
        None,
    )?;

    interchain.check_ibc(&src_chain_id, tx_response)?;

    Ok(())
}
