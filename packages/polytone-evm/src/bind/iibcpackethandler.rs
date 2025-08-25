use alloy::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IIBCPacketHandler,
    "./solidity/out/IIBCPacketHandler.sol/IIBCPacketHandler.json"
);
