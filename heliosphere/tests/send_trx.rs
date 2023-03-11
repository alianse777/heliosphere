use heliosphere::RpcClient;
use heliosphere_core::Address;
use heliosphere_signer::{keypair::Keypair, signer::Signer};

#[tokio::test]
async fn test_send_trx() {
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
    let amount = 1;
    let old_balance = client.get_account_balance(&from).await.unwrap();
    let mut tx = client.trx_transfer(&from, &to, amount).await.unwrap();
    keypair.sign_transaction(&mut tx).unwrap();
    let txid = client.broadcast_transaction(&tx).await.unwrap();
    println!("Txid: {}", txid);
    println!("Confirming...");
    let info = client.await_confirmation(txid).await.unwrap();
    println!("{:?}", info);
    let new_balance = client.get_account_balance(&from).await.unwrap();
    assert!(old_balance >= new_balance + amount); // including TRX burn
}
