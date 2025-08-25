use cw_orch::interface;

pub const EVM_NOTE: &str = "abstract:evm-note";

#[interface(
    crate::msg::InstantiateMsg,
    crate::msg::ExecuteMsg,
    crate::msg::QueryMsg,
    crate::msg::MigrateMsg,
    id = EVM_NOTE
)]
pub struct EvmNote<Chain>;

#[cfg(not(target_arch = "wasm32"))]
use cw_orch::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
impl<Chain: CwEnv> Uploadable for EvmNote<Chain> {
    fn wrapper() -> <Mock as TxHandler>::ContractSource {
        Box::new(
            ContractWrapper::new(
                crate::contract::execute,
                crate::contract::instantiate,
                crate::contract::query,
            )
            .with_reply(crate::ibc::reply)
            .with_ibc(
                crate::ibc::ibc_channel_open,
                crate::ibc::ibc_channel_connect,
                crate::ibc::ibc_channel_close,
                crate::ibc::ibc_packet_receive,
                crate::ibc::ibc_packet_ack,
                crate::ibc::ibc_packet_timeout,
            ),
        )
    }
    fn wasm(_chain_info: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("evm_note")
            .unwrap()
    }
}
