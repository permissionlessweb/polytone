use alloy::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    EvmVoice,
    "./solidity/out/EvmVoice.sol/EvmVoice.json"
);
