use ethabi::{ethereum_types::U256, ParamType, Token};
use heliosphere::{MethodCall, RpcClient};
use heliosphere_core::Address;
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
    let client = RpcClient::new(api).unwrap();
    let from = keypair.address();
    let to: Address = "TB9n2jzcWoqta1xX2Mv8P3y9tyUNsGTFsQ".parse().unwrap();
    let usdt: Address = "TG3XXyExBkPp9nzdajDZsozEu4BkaSJozs".parse().unwrap();
    let amount: u64 = 1; // 0.000001 USDT

    // Fetch account balance before
    let method_call = MethodCall {
        caller: &from,
        contract: &usdt,
        selector: "balanceOf(address)",
        parameter: &ethabi::encode(&[Token::Address(from.into())]),
    };
    let res = &ethabi::decode(
        &[ParamType::Uint(256)],
        &client
            .query_contract(&method_call)
            .await
            .unwrap()
            .constant_result(0)
            .unwrap(),
    )
    .unwrap()[0];
    match res {
        Token::Uint(x) => println!("Balance: {}", x),
        _ => panic!("Wrong type"),
    }

    let method_call = MethodCall {
        caller: &from,
        contract: &usdt,
        selector: "transfer(address,uint256)",
        parameter: &ethabi::encode(&[Token::Address(to.into()), Token::Uint(U256::from(amount))]),
    };
    // Estimate energy usage
    let estimated = client.estimate_energy(&method_call).await.unwrap();
    println!("Estimated energy usage: {}", estimated);

    // Send token
    let mut tx = client
        .trigger_contract(&method_call, 0, None)
        .await
        .unwrap();
    keypair.sign_transaction(&mut tx).unwrap();
    let txid = client.broadcast_transaction(&tx).await.unwrap();
    println!("Txid: {}", txid);
    println!("Confirming...");
    client.await_confirmation(txid).await.unwrap();

    // Fetch account balance after
    let method_call = MethodCall {
        caller: &from,
        contract: &usdt,
        selector: "balanceOf(address)",
        parameter: &ethabi::encode(&[Token::Address(from.into())]),
    };
    let res = &ethabi::decode(
        &[ParamType::Uint(256)],
        &client
            .query_contract(&method_call)
            .await
            .unwrap()
            .constant_result(0)
            .unwrap(),
    )
    .unwrap()[0];
    match res {
        Token::Uint(x) => println!("Balance: {}", x),
        _ => panic!("Wrong type"),
    }
}
