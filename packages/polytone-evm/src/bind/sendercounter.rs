use alloy::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    SenderCounter,
    "./solidity/out/SenderCounter.sol/SenderCounter.json"
);
