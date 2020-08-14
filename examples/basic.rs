// use futures::executor::block_on;
// use futures::join;
// use std::io;

extern crate vault_rs as vault;
use vault::Vault;

#[tokio::main]
async fn main() {
    let host = "http://localhost:8200";
    let token = "test12345";

    let client = vault::Client::from_environment::<&str, &str, &str>(
        Some(host),
        Some(token),
        None,
    )
    .unwrap();
    println!("client address is {}", client.address());
    println!("client is {:#?}", client);

    let secret = client.get("secret/data/my-secret").await.unwrap();
    println!("secret is {:#?}", secret)
}
