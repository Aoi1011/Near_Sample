use solana_client::nonblocking::rpc_client::RpcClient;

#[tokio::main]
async fn main() {
    let url = "http://localhost:8899".to_string();
    let rpc_client = RpcClient::new(url);
    let block = rpc_client.get_block(1).await.expect("get block");

    eprintln!("Block: {:?}", block);
}
