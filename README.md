![Crates.io](https://img.shields.io/crates/v/heliosphere?style=flat-square) ![Crates.io](https://img.shields.io/crates/l/heliosphere?style=flat-square)

## Description

Rust-idiomatic Tron API client library.

## Supported features

| Features | Support |
|----------|---------|
| Transaction signing & broadcasting | &check; |
| Smart contract calls | &check; |
| Basic network querying | &check; |
| Staking TRX for energy and bandwidth | &check; |
| Offline transaction signing | &check; |
| Offline transaction encoding (without CreateTransaction API) | &cross; |
| Voting & Proposals | &cross; |

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
let to: Address = "<transfer-to-address>".parse().unwrap();
let usdt: Address = "TG3XXyExBkPp9nzdajDZsozEu4BkaSJozs".parse().unwrap(); // shasta testnet USDT
let amount: u64 = 1; // 0.000001 USDT

// Fetch account balance
let method_call_balance = MethodCall {
    caller: &from,
    contract: &usdt,
    selector: "balanceOf(address)",
    parameter: &ethabi::encode(&[Token::Address(from.into())]),
};
let res = &ethabi::decode(
    &[ParamType::Uint(256)],
    &client
        .query_contract(&method_call_balance)
        .await
        .unwrap()
        .constant_result(0)
        .unwrap(),
)
.unwrap()[0];
let current_balance = match res {
    Token::Uint(x) => x,
    _ => panic!("Wrong type"),
};
println!("Balance: {}", current_balance);

// Transfer tokens
let method_call = MethodCall {
    caller: &from,
    contract: &usdt,
    selector: "transfer(address,uint256)",
    parameter: &ethabi::encode(&[Token::Address(to.into()), Token::Uint(U256::from(amount))]),
};
// Estimate energy usage
let estimated = client.estimate_energy(&method_call).await.unwrap();
println!("Estimated energy usage: {}", estimated);
// Send tx
let mut tx = client
    .trigger_contract(&method_call, 0, None)
    .await
    .unwrap();
keypair.sign_transaction(&mut tx).unwrap();
let txid = client.broadcast_transaction(&tx).await.unwrap();
println!("Txid: {}", txid);
println!("Confirming...");
client.await_confirmation(txid).await.unwrap();
```

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/tokio-rs/tokio/blob/master/LICENSE