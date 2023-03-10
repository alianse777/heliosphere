use heliosphere::{ResourceType, RpcClient};
use heliosphere_core::Address;
use heliosphere_signer::{keypair::Keypair, signer::Signer};

#[tokio::test]
async fn test_freeze_balance() {
    let api = "https://api.shasta.trongrid.io";
    let keypair = Keypair::from_hex_key(
        std::fs::read_to_string(".key")
            .expect("no ./.key found")
            .trim(),
    )
    .unwrap();
    let client = RpcClient::new(api).unwrap();
    let from = keypair.address();
    let to: Address = "TB9n2jzcWoqta1xX2Mv8P3y9tyUNsGTFsQ".parse().unwrap();
    let old_balance = client.get_account_balance(&from, None).await.unwrap();
    let amount = 1_000_000;
    // For owner
    let mut tx = client
        .freeze_balance(&from, amount, ResourceType::Energy, None)
        .await
        .unwrap();
    keypair.sign_transaction(&mut tx).unwrap();
    let txid = client.broadcast_transaction(&tx).await.unwrap();
    println!("{}", txid);
    // For another address
    let mut tx = client
        .freeze_balance(&from, amount, ResourceType::Bandwidth, Some(&to))
        .await
        .unwrap();
    keypair.sign_transaction(&mut tx).unwrap();
    let txid = client.broadcast_transaction(&tx).await.unwrap();
    println!("{}", txid);
    println!("Confirming...");
    let info = client.await_confirmation(txid).await.unwrap();
    println!("{:?}", info);
    let new_balance = client.get_account_balance(&from, None).await.unwrap();
    assert_eq!(old_balance, new_balance + amount);
}
