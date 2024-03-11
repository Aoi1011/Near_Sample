use anyhow::Context;
use solana_client::nonblocking::rpc_client::RpcClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "".to_string();
    let rpc_client = RpcClient::new(url);
    // let block = rpc_client.get_block(1).await.expect("get block");

    let vote_accounts = rpc_client
        .get_vote_accounts()
        .await
        .context("get vote accounts")?;

    eprintln!("Vote accounts: {}", vote_accounts.current.len());



    // eprintln!("Block: {:?}", block);

    Ok(())
}
