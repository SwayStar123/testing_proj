use fuels::{prelude::*, types::{Bits256, ContractId}};

// Load abi from json
abigen!(
    Contract(name = "MyContract", abi = "proj1/out/debug/proj1-abi.json"),
    Contract(
        name = "ProxyContract",
        abi = "proj2/out/debug/proj2-abi.json"
    )
);

async fn get_wallet() -> WalletUnlocked {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await
    .unwrap();
    let wallet = wallets.pop().unwrap();
    wallet
}

async fn get_contract_instance(wallet: WalletUnlocked) -> (MyContract<WalletUnlocked>, ContractId) {
    let id = Contract::load_from("proj1/out/debug/proj1.bin", LoadConfiguration::default())
        .unwrap()
        .deploy(&wallet, TxPolicies::default())
        .await
        .unwrap();

    let instance = MyContract::new(id.clone(), wallet);

    (instance, id.into())
}

async fn get_proxy_contract_instance(
    wallet: WalletUnlocked,
    target_contract: ContractId,
) -> (ProxyContract<WalletUnlocked>, ContractId) {
    let id = Contract::load_from(
        "proj2/out/debug/proj2.bin",
        LoadConfiguration::default().with_configurables(
            ProxyContractConfigurables::default()
                .with_TARGET_CONTRACT(target_contract)
                .unwrap(),
        ),
    )
    .unwrap()
    .deploy(&wallet, TxPolicies::default())
    .await
    .unwrap();

    let instance = ProxyContract::new(id.clone(), wallet);

    (instance, id.into())
}

#[tokio::test]
async fn test_proxy() {
    let wallet = get_wallet().await;
    let (_target_instance, target_id) = get_contract_instance(wallet.clone()).await;
    let (proxy_instance, _proxy_id) = get_proxy_contract_instance(wallet, target_id).await;

    let b256_1 = Bits256::from_hex_str("0x00000000000000000000000059F2f1fCfE2474fD5F0b9BA1E73ca90b143Eb8d0").unwrap();

    let result = proxy_instance
        .methods()
        .test_function()
        .with_contract_ids(&[target_id.into()])
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(result, b256_1);
}

#[tokio::test]
async fn test_proxy2() {
    let wallet = get_wallet().await;
    let (_target_instance, target_id) = get_contract_instance(wallet.clone()).await;
    let (proxy_instance, _proxy_id) = get_proxy_contract_instance(wallet, target_id).await;
    let b256_2 = Bits256::from_hex_str("0x0000000000000000000000001111111111111111111111111111111111111111").unwrap();

    
    let result = proxy_instance
        .methods()
        .test_function_2()
        .with_contract_ids(&[target_id.into()])
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(result, b256_2);
}

