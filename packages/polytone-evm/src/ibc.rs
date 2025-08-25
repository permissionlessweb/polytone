use crate::evm::abi_types::{
    EvmMsg as IEvmMsg, EvmMsgType as IEvmMsgType, ExecuteResponsePacket as IExecuteResponsePacket,
    ExecuteResult as IExecuteResult, Msg as IMsg, MsgType as IMsgType, Packet as IPacket,
};
use alloy::primitives::Address;
use alloy_sol_types::SolValue;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::HexBinary;
use msg_data::MsgData as _;

use crate::evm::EvmMsg;

pub const VERSION: &str = "polytone-evm-1";

#[cw_serde]
pub struct Packet {
    /// Message sender on the note chain.
    pub sender: String,
    /// Message to execute on voice chain.
    pub msg: Msg,
}

impl Packet {
    pub fn encode(self) -> Result<Vec<u8>, alloy_sol_types::Error> {
        Ok(IPacket {
            sender: self.sender.clone(),
            msg: self.msg.encode()?,
        }
        .abi_encode())
    }
    pub fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, alloy_sol_types::Error> {
        let packet = IPacket::abi_decode(bytes.as_ref())?;
        Ok(Packet {
            sender: packet.sender,
            msg: Msg::decode(&packet.msg)?,
        })
    }
}

#[cw_serde]
/// Message sent to EVM Voice contract
pub enum Msg {
    /// Execute a message on the EVM
    Execute { msgs: Vec<EvmMsg<String>> },
}

#[cw_serde]
pub struct ExecuteResponsePacket {
    pub executed_by: String,
    pub result: Vec<ExecuteResult>,
}

impl ExecuteResponsePacket {
    pub fn decode_bytes(bz: impl AsRef<[u8]>) -> Result<Self, alloy_sol_types::Error> {
        let packet = IExecuteResponsePacket::abi_decode(bz.as_ref())?;
        Ok(ExecuteResponsePacket {
            executed_by: packet.executedBy.to_checksum(None),
            result: packet
                .result
                .into_iter()
                .map(|res| ExecuteResult {
                    success: res.success,
                    data: HexBinary::from(res.data.to_vec()),
                })
                .collect(),
        })
    }
}

#[cw_serde]
pub struct ExecuteResult {
    pub success: bool,
    pub data: HexBinary,
}

pub mod msg_data {
    use alloy::primitives::Bytes;
    use cosmwasm_std::HexBinary;

    use crate::evm::abi_types::CallMessage;

    use super::*;

    /// Trait that identifies encoding messages for EVM
    pub trait MsgData: Sized {
        type EvmType: SolValue;

        fn encode(self) -> Result<Self::EvmType, alloy_sol_types::Error> {
            unimplemented!("Type does not support encoding");
        }
        #[allow(unused_variables)]
        fn decode(evm_type: &Self::EvmType) -> Result<Self, alloy_sol_types::Error> {
            unimplemented!("Type does not support decoding");
        }
    }

    impl MsgData for EvmMsg<Address> {
        type EvmType = IEvmMsg;

        fn encode(self) -> Result<Self::EvmType, alloy_sol_types::Error> {
            match &self {
                EvmMsg::Call { .. } => Ok(IEvmMsg {
                    msgType: IEvmMsgType::Call,
                    message: self.encode().into(),
                }),
                EvmMsg::DelegateCall { .. } => Ok(IEvmMsg {
                    msgType: IEvmMsgType::DelegateCall,
                    message: self.encode().into(),
                }),
            }
        }
    }

    impl MsgData for EvmMsg<String> {
        type EvmType = IEvmMsg;

        fn decode(evm_msg: &IEvmMsg) -> Result<Self, alloy_sol_types::Error> {
            match evm_msg.msgType {
                IEvmMsgType::Call => {
                    let call_msg = CallMessage::abi_decode(evm_msg.message.as_ref())?;
                    Ok(EvmMsg::Call {
                        to: call_msg.to.to_string(),
                        data: HexBinary::from(call_msg.data.to_vec()),
                        allow_failure: Some(call_msg.allowFailure),
                        value: Some(call_msg.value.to::<u128>().into()),
                    })
                }
                IEvmMsgType::DelegateCall => {
                    let call_msg = CallMessage::abi_decode(evm_msg.message.as_ref())?;
                    Ok(EvmMsg::DelegateCall {
                        to: call_msg.to.to_string(),
                        data: HexBinary::from(call_msg.data.to_vec()),
                        allow_failure: Some(call_msg.allowFailure),
                        value: Some(call_msg.value.to::<u128>().into()),
                    })
                }
                IEvmMsgType::__Invalid => panic!(),
            }
        }
    }

    impl MsgData for Msg {
        type EvmType = IMsg;

        fn encode(self) -> Result<Self::EvmType, alloy_sol_types::Error> {
            match self {
                Msg::Execute { msgs } => Ok(IMsg {
                    msgType: IMsgType::Execute,
                    data: msgs
                        .into_iter()
                        .map(|m| m.check().unwrap().encode().into())
                        .collect::<Vec<Bytes>>(),
                }),
            }
        }

        fn decode(msg: &IMsg) -> Result<Self, alloy_sol_types::Error> {
            match msg.msgType {
                IMsgType::Execute => {
                    let mut msgs = Vec::with_capacity(msg.data.len());
                    for data in &msg.data {
                        let evm_msg = IEvmMsg::abi_decode(data)?;
                        msgs.push(EvmMsg::decode(&evm_msg)?);
                    }
                    Ok(Msg::Execute { msgs })
                }
                IMsgType::__Invalid => panic!(),
            }
        }
    }

    impl MsgData for ExecuteResponsePacket {
        type EvmType = IExecuteResponsePacket;
        fn decode(evm_type: &Self::EvmType) -> Result<Self, alloy_sol_types::Error> {
            let mut results = Vec::with_capacity(evm_type.result.len());
            for res in &evm_type.result {
                results.push(ExecuteResult::decode(res)?);
            }
            Ok(ExecuteResponsePacket {
                executed_by: evm_type.executedBy.to_string(),
                result: results,
            })
        }
    }

    impl MsgData for ExecuteResult {
        type EvmType = IExecuteResult;
        fn decode(evm_type: &Self::EvmType) -> Result<Self, alloy_sol_types::Error> {
            Ok(ExecuteResult {
                success: evm_type.success,
                data: HexBinary::from(evm_type.data.to_vec()),
            })
        }
    }
}
