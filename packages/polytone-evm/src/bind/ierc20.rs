use alloy::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20,
    "./solidity/out/ERC20/IERC20.sol/IERC20.json"
);
