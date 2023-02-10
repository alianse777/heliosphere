![Crates.io](https://img.shields.io/crates/v/heliopshere?style=flat-square) ![Crates.io](https://img.shields.io/crates/l/heliosphere?style=flat-square)

## Description

Rust-idiomatic Tron API client library.

## Supported features

| Features | Support |
|----------|---------|
| Transaction signing & broadcasting | [x] |
| Smart contract calls | [x] |
| Basic network querying | [x] |
| Staking TRX for energy and bandwidth | [x] |
| Offline transaction signing | [x] |
| Offline transaction encoding (without CreateTransaction API) | [] |
| Voting & Proposals | [] |

## Structure

| Crate         | Description     |
|--------------|------------------|
| [heliosphere](https://crates.io/crates/heliosphere) | Main crate |
| [heliosphere-core](https://crates.io/crates/heliosphere-core) | Core types |
| [heliosphere-signer](https://crates.io/crates/heliosphere-signer) | Transaction signing utils |

## TRC20 transfer example

```
let api = "https://api.shasta.trongrid.io";
let keypair = Keypair::from_hex_key(
    std::fs::read_to_string(".key")
        .expect("no ./.key found")
        .trim(),
)
.unwrap();
let client = RpcClient::new(api).unwrap();
let from = keypair.address();
let to: Address = "<address_to>".parse().unwrap();
let usdt: Address = "TG3XXyExBkPp9nzdajDZsozEu4BkaSJozs".parse().unwrap();
let amount: u64 = 1; // 0.000001 USDT 

let method_call = MethodCall {
    caller: &from,
    contract: &usdt,
    selector: "transfer(address,uint256)",
    parameter: &ethabi::encode(&[Token::Address(to.into()), Token::Uint(U256::from(amount))]),
};
let tx = client
    .trigger_contract(
        &method_call,
        client.estimate_fee_limit(&method_call).await.unwrap(),
        0,
    )
    .await
    .unwrap();
keypair.sign_transaction(&mut tx).unwrap();
let txid = client.broadcast_transaction(&tx).await.unwrap();
println!("Txid: {}", txid);
println!("Confirming...");
client.await_confirmation(txid).await.unwrap();
```
