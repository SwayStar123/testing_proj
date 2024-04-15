contract;

abi MyContract {
    fn test_function() -> b256;
    fn test_function_2() -> b256;
}

impl MyContract for Contract {
    fn test_function() -> b256 {
        0x00000000000000000000000059F2f1fCfE2474fD5F0b9BA1E73ca90b143Eb8d0
    }

    fn test_function_2() -> b256 {
        0x0000000000000000000000001111111111111111111111111111111111111111
    }
}
