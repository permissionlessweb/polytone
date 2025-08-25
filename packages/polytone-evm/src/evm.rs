use crate::erc20::Erc20Token;
use alloy::primitives::{Address, AddressError, U256};
use alloy_sol_types::SolValue;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{HexBinary, Uint128};

/// Marks either `String` or valid EVM `Address`.
///
/// String is used in unverified types, such as messages and query responses.
/// Addr is used in verified types, which are to be stored in blockchain state.
///
/// This trait is intended to be used as a generic in type definitions.
pub trait EvmAddressLike {}

impl EvmAddressLike for String {}

impl EvmAddressLike for HexBinary {}

impl EvmAddressLike for Address {}

/// A message to be sent to the EVM
#[cw_serde]
#[non_exhaustive]
pub enum EvmMsg<T: EvmAddressLike> {
    /// Call a contract with the given data
    Call {
        /// The address of the contract to call, must pass checksum validation
        to: T,
        /// The calldata to send to the contract
        data: HexBinary,
        /// Native ETH used in the tx
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<Uint128>,
        /// Don't revert entire batch when this transaction fails
        #[serde(skip_serializing_if = "Option::is_none")]
        allow_failure: Option<bool>,
    },
    /// Delegate a call to an external contract. This is dangerous and should be used with caution.
    DelegateCall {
        /// The address of the contract to call, must pass checksum validation
        to: T,
        /// The calldata to send to the contract
        data: HexBinary,
        /// Native ETH used in the tx
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<Uint128>,
        /// Don't revert entire batch when this transaction fails
        #[serde(skip_serializing_if = "Option::is_none")]
        allow_failure: Option<bool>,
    },
}

impl<T> EvmMsg<T>
where
    T: EvmAddressLike,
{
    pub fn call(to: T, data: impl Into<HexBinary>) -> Self {
        EvmMsg::Call {
            to,
            data: data.into(),
            value: None,
            allow_failure: None,
        }
    }

    pub fn call_with_value(to: T, data: impl Into<HexBinary>, value: Uint128) -> Self {
        EvmMsg::Call {
            to,
            data: data.into(),
            value: Some(value),
            allow_failure: None,
        }
    }

    pub fn delegate_call(to: T, data: impl Into<HexBinary>) -> Self {
        EvmMsg::DelegateCall {
            to,
            data: data.into(),
            value: None,
            allow_failure: None,
        }
    }
}

impl EvmMsg<String> {
    /// Check the validity of the addresses
    pub fn check(self) -> Result<EvmMsg<Address>, AddressError> {
        match self {
            EvmMsg::Call {
                to,
                data,
                value,
                allow_failure,
            } => {
                let to: Address = Address::parse_checksummed(to, None)?;
                Ok(EvmMsg::Call {
                    to,
                    data,
                    value,
                    allow_failure,
                })
            }
            EvmMsg::DelegateCall {
                to,
                data,
                value,
                allow_failure,
            } => {
                let to: Address = Address::parse_checksummed(to, None)?;
                Ok(EvmMsg::DelegateCall {
                    to,
                    data,
                    value,
                    allow_failure,
                })
            }
        }
    }
}

impl EvmMsg<Address> {
    pub fn encode(self) -> Vec<u8> {
        match self {
            EvmMsg::Call {
                to,
                data,
                allow_failure,
                value,
            } => {
                let data = data.to_vec();
                let call = abi_types::CallMessage {
                    to,
                    data: data.to_vec().into(),
                    allowFailure: allow_failure.unwrap_or(false),
                    value: value.map(|v| U256::from(v.u128())).unwrap_or_default(),
                }
                .abi_encode();
                abi_types::EvmMsg {
                    msgType: abi_types::EvmMsgType::Call,
                    message: call.into(),
                }
                .abi_encode()
            }
            EvmMsg::DelegateCall {
                to,
                data,
                allow_failure,
                value,
            } => {
                let data = data.to_vec();
                let call = abi_types::CallMessage {
                    to,
                    data: data.to_vec().into(),
                    allowFailure: allow_failure.unwrap_or(false),
                    value: value.map(|v| U256::from(v.u128())).unwrap_or_default(),
                }
                .abi_encode();
                abi_types::EvmMsg {
                    msgType: abi_types::EvmMsgType::DelegateCall,
                    message: call.into(),
                }
                .abi_encode()
            }
        }
    }

    #[allow(dead_code)]
    fn unchecked(self) -> EvmMsg<String> {
        match self {
            EvmMsg::Call {
                to,
                data,
                allow_failure,
                value,
            } => EvmMsg::Call {
                to: to.to_string(),
                data,
                allow_failure,
                value,
            },
            EvmMsg::DelegateCall {
                to,
                data,
                allow_failure,
                value,
            } => EvmMsg::DelegateCall {
                to: to.to_string(),
                data,
                allow_failure,
                value,
            },
        }
    }
}

pub(crate) mod abi_types {
    use alloy_sol_types::sol;

    // Copied directly from solidity/src/Requests.sol
    sol! {
    // Reflect the CW Packet
    struct Packet {
        string sender;
        Msg msg;
    }

    // Message called on the voice
    struct Msg {
        MsgType msgType;
        // Requests on the voice (Exec / Query)
        bytes[] data;
    }

    // Type of voice message
    enum MsgType {
        Execute
    //    Query
    }

    // Message called on the proxy contract
    // Reflect EvmMsg<Address>
    struct EvmMsg {
        EvmMsgType msgType;
        bytes message;
    }

    // Type of message called on proxy contract
    enum EvmMsgType {
        Call,
        DelegateCall,
    }

    // Data to execute by proxy
    struct CallMessage {
        address to;
        bool allowFailure;
        uint256 value;
        bytes data;
    }


    struct ExecuteResult {
        bool success;
        bytes data;
    }

    struct ExecuteResponsePacket {
        address executedBy;
        ExecuteResult[] result;
    }


    struct Token {
        address denom;
        uint128 amount;
    }

    }
}

impl From<abi_types::Token> for Erc20Token<String> {
    fn from(token: abi_types::Token) -> Self {
        Erc20Token {
            address: token.denom.to_string(),
            amount: token.amount.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod call_message {
        use crate::ibc::{Msg, Packet};

        use super::*;

        #[test]
        fn unchecked_encoding() {
            let msg = abi_types::CallMessage {
                to: "0x785B548D3d7064F77A26e479AC7847DBCE0c1B46"
                    .parse()
                    .unwrap(),
                data: HexBinary::from_hex("b49004e9").unwrap().to_vec().into(),
                allowFailure: false,
                value: U256::from(0),
            };

            let encoded = msg.abi_encode();
            let actual = HexBinary::from(encoded);

            let expected = HexBinary::from_hex("0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000785b548d3d7064f77a26e479ac7847dbce0c1b460000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000004b49004e900000000000000000000000000000000000000000000000000000000").unwrap();

            assert_eq!(actual, expected);
        }

        #[test]
        fn checked_encoding() {
            let msg = EvmMsg::call(
                "0x785B548D3d7064F77A26e479AC7847DBCE0c1B46".to_string(),
                HexBinary::from_hex("b49004e9").unwrap(),
            )
            .check()
            .unwrap();

            let encoded = msg.encode();
            let actual = HexBinary::from(encoded);

            let expected = HexBinary::from_hex("0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000785b548d3d7064f77a26e479ac7847dbce0c1b460000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000004b49004e900000000000000000000000000000000000000000000000000000000").unwrap();

            assert_eq!(actual, expected);
        }

        const TEST_SENDER: &str =
            "union1tw2y3uk7fwcjeh208gdwaqd3utkuzmnp2fyvqve00jg8vtyhuhfqsender";

        #[test]
        fn packet_encoding() {
            let evm_msg = EvmMsg::call(
                "0x785B548D3d7064F77A26e479AC7847DBCE0c1B46".to_string(),
                HexBinary::from_hex("b49004e9").unwrap(),
            );

            let msg = Msg::Execute {
                msgs: vec![evm_msg],
            };
            let request: Packet = Packet {
                msg,
                sender: TEST_SENDER.to_string(),
            };
            let encoded = request.encode().unwrap();
            let actual = HexBinary::from(encoded);

            let expected = HexBinary::from_hex("0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000040756e696f6e317477327933756b376677636a65683230386764776171643375746b757a6d6e703266797671766530306a6738767479687568667173656e6465720000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000016000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000e00000000000000000000000000000000000000000000000000000000000000020000000000000000000000000785b548d3d7064f77a26e479ac7847dbce0c1b460000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000004b49004e900000000000000000000000000000000000000000000000000000000").unwrap();

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_address() {
        EvmMsg::call(
            "0x785B548D3d7064F77A26e479AC7847DBCE0c1B46".to_string(),
            HexBinary::from_hex("b49004e9").unwrap(),
        );
    }
}
