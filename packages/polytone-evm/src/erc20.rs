use alloy::primitives::Address;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

use crate::evm::EvmAddressLike;

#[cw_serde]
pub struct Erc20Token<T: EvmAddressLike> {
    pub address: T,
    pub amount: Uint128,
}

impl Erc20Token<Address> {
    pub fn unchecked(self) -> Erc20Token<String> {
        Erc20Token {
            address: self.address.to_string(),
            amount: self.amount,
        }
    }
}
