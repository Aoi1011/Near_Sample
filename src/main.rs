use anyhow::Context;
use dotenv::dotenv;
use solana_client::nonblocking::rpc_client::RpcClient;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let url = std::env::var("RPC_URL").context("read RPC_URL")?;

    let rpc_client = RpcClient::new(url);

    // let block = rpc_client.get_block(1).await.expect("get block");

    // let vote_accounts = rpc_client
    //     .get_vote_accounts()
    //     .await
    //     .context("get vote accounts")?;

    // eprintln!("Vote accounts: {}", vote_accounts.current.len());

    let start_slot = 0;
    let limit = 3;
    let confirmed_blocks = rpc_client.get_blocks(start_slot, Some(500_000)).await.context("")?;

    eprintln!("Got Confirmed blocks, the length is {}", confirmed_blocks.len());

    // let block = rpc_client.get_block(0).await.context("get block")?;

    // eprintln!("Block: {:?}", block);

    // let listener = TcpListener::bind("127.0.0.1:2345")
    //     .await
    //     .context("binding")?;
    // while let Ok(stream) = listener.accept().await {}

    // eprintln!("Block: {:?}", block);

    Ok(())
}
