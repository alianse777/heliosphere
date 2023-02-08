use heliosphere::RpcClient;
use heliosphere_signer::{keypair::Keypair, signer::Signer};

#[tokio::main]
async fn main() {
    let api = "https://api.shasta.trongrid.io";
    let keypair = Keypair::from_hex_key(
        std::fs::read_to_string(".key")
            .expect("no ./.key found")
            .trim(),
    )
    .unwrap();
    let account = keypair.address();
    let client = RpcClient::new(api).unwrap();
    let resources = client.get_account_resources(&account).await.unwrap();
    println!("{:?}", resources);
}
