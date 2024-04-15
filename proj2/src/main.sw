contract;

use std::{
    execution::run_external,
    constants::ZERO_B256,
};

configurable {
    TARGET_CONTRACT: ContractId = ContractId::from(ZERO_B256),
}

abi MyContract {
    fn test_function() -> b256;
    fn test_function_2() -> b256;
}

impl MyContract for Contract {
    fn test_function() -> b256 {
        run_external(TARGET_CONTRACT)
    }

    fn test_function_2() -> b256 {
        run_external(TARGET_CONTRACT)
    }
}
