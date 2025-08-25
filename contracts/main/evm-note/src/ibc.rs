#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    DepsMut, Env, HexBinary, IbcBasicResponse, IbcChannelCloseMsg, IbcChannelConnectMsg,
    IbcChannelOpenMsg, IbcChannelOpenResponse, IbcPacketAckMsg, IbcPacketReceiveMsg,
    IbcPacketTimeoutMsg, IbcReceiveResponse, Never, Reply, Response, SubMsg,
};
use polytone_evm::{accounts, ack::Ack, callbacks, handshake::note, ibc::Packet};

use crate::{
    error::ContractError,
    state::{BLOCK_MAX_GAS, CHANNEL, CONNECTION_REMOTE_PORT},
};

// TODO: Remove before production
pub(crate) mod temp_test {
    use cosmwasm_schema::cw_serde;
    use cosmwasm_std::HexBinary;
    use cw_storage_plus::Map;
    use polytone_evm::ibc::ExecuteResponsePacket;

    use super::*;

    #[cw_serde]
    pub enum SentPacketData {
        Binary(HexBinary),
        Request(Packet),
    }

    #[cw_serde]
    pub enum ReceivedPacketData {
        Binary(HexBinary),
        Result(ExecuteResponsePacket),
    }

    #[cw_serde]
    pub struct AckInfo {
        pub height: u64,
        pub index: Option<u32>,
        pub sent: SentPacketData,
        pub result: Ack,
    }

    pub const ACK_DATAS: Map<u64, AckInfo> = Map::new("ack_datas");
}

/// The amount of gas that needs to be reserved for handling a
/// callback error in the reply method. See `TestNoteOutOfGas` in the
/// simulation tests for a test that can be used to tune thise.
pub(crate) const ERR_GAS_NEEDED: u64 = 101_000;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_open(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelOpenMsg,
) -> Result<IbcChannelOpenResponse, ContractError> {
    let response = note::open(&msg, &["EVM"])?;
    match CONNECTION_REMOTE_PORT.may_load(deps.storage)? {
        Some((conn, port)) => {
            if msg.channel().counterparty_endpoint.port_id != port
                || msg.channel().connection_id != conn
            {
                Err(ContractError::AlreadyPaired {
                    suggested_connection: msg.channel().connection_id.clone(),
                    suggested_port: msg.channel().counterparty_endpoint.port_id.clone(),
                    pair_connection: conn,
                    pair_port: port,
                })
            } else {
                Ok(response)
            }
        }
        None => Ok(response),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_connect(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelConnectMsg,
) -> Result<IbcBasicResponse, ContractError> {
    note::connect(&msg, &["EvmMsg"])?;
    CONNECTION_REMOTE_PORT.save(
        deps.storage,
        &(
            msg.channel().connection_id.clone(),
            msg.channel().counterparty_endpoint.port_id.clone(),
        ),
    )?;
    CHANNEL.save(deps.storage, &msg.channel().endpoint.channel_id)?;
    Ok(IbcBasicResponse::new()
        .add_attribute("method", "ibc_channel_connect")
        .add_attribute("channel_id", &msg.channel().endpoint.channel_id))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_close(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelCloseMsg,
) -> Result<IbcBasicResponse, ContractError> {
    CHANNEL.remove(deps.storage);
    Ok(IbcBasicResponse::default()
        .add_attribute("method", "ibc_channel_close")
        .add_attribute("connection_id", msg.channel().connection_id.clone())
        .add_attribute(
            "counterparty_port_id",
            msg.channel().counterparty_endpoint.port_id.clone(),
        ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_receive(
    _deps: DepsMut,
    _env: Env,
    _msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, Never> {
    unreachable!("voice should never send a packet")
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_ack(
    deps: DepsMut,
    _env: Env,
    ack: IbcPacketAckMsg,
) -> Result<IbcBasicResponse, ContractError> {
    let maybe_packet = Packet::decode(&ack.original_packet.data);

    // TODO: Remove error catching for production
    let packet = match maybe_packet {
        Ok(decoded) => temp_test::SentPacketData::Request(decoded),
        Err(err) => {
            deps.api.debug(&format!("Error decoding packet: {:?}", err));
            temp_test::SentPacketData::Binary(HexBinary::from(ack.original_packet.data.clone()))
        }
    };

    let (callback, executed_by, result) = callbacks::on_ack(deps.storage, &ack);

    let callback_msg = callback.map(|callback| {
        SubMsg::reply_on_error(callback, ack.original_packet.sequence).with_gas_limit(
            BLOCK_MAX_GAS
                .load(deps.storage)
                .expect("set during instantiation")
                - ERR_GAS_NEEDED,
        )
    });

    accounts::on_ack(
        deps.storage,
        ack.original_packet.src.channel_id.clone(),
        ack.original_packet.sequence,
        executed_by,
    );

    let ack_info = temp_test::AckInfo {
        height: _env.block.height,
        index: _env.transaction.map(|tx| tx.index),
        sent: packet,
        result,
    };

    temp_test::ACK_DATAS
        .save(deps.storage, ack.original_packet.sequence, &ack_info)
        .ok();

    Ok(IbcBasicResponse::default()
        .add_attribute("method", "ibc_packet_ack")
        .add_attribute("sequence_number", ack.original_packet.sequence.to_string())
        .add_submessages(callback_msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_timeout(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
    let callback = callbacks::on_timeout(deps.storage, &msg).map(|cosmos_msg| {
        SubMsg::reply_on_error(cosmos_msg, msg.packet.sequence).with_gas_limit(
            BLOCK_MAX_GAS
                .load(deps.storage)
                .expect("set during instantiation")
                - ERR_GAS_NEEDED,
        )
    });

    accounts::on_timeout(deps.storage, msg.packet.src.channel_id, msg.packet.sequence);

    Ok(IbcBasicResponse::default()
        .add_attribute("method", "ibc_packet_timeout")
        .add_attribute("sequence_number", msg.packet.sequence.to_string())
        .add_submessages(callback))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    let sequence = msg.id;
    Ok(Response::default()
        .add_attribute("method", "reply_callback_error")
        .add_attribute("packet_sequence", sequence.to_string())
        .add_attribute("callback_error", msg.result.unwrap_err()))
}
