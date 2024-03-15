use anyhow::Context;
use dotenv::dotenv;
use solana_client::nonblocking::rpc_client::RpcClient;
use tokio::net::TcpListener;

const ONE_EPOCH: u64 = 432_000;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let url = std::env::var("RPC_URL").context("read RPC_URL")?;

    let rpc_client = RpcClient::new(url);

    let latest_slot = rpc_client.get_slot().await.expect("get slot");

    // let vote_accounts = rpc_client
    //     .get_vote_accounts()
    //     .await
    //     .context("get vote accounts")?;

    // eprintln!("Vote accounts: {}", vote_accounts.current.len());

    let start_slot = 0;
    let limit = 3;
    let mut rewards = Vec::new();

    loop {
        let end_slot = start_slot + ONE_EPOCH;

        let confirmed_blocks = rpc_client
            .get_blocks(start_slot, Some(end_slot))
            .await
            .context("get confirmed blocks")?;

        eprintln!(
            "Got Confirmed blocks, the length is {}",
            confirmed_blocks.len()
        );

        for confimed_blockn in confirmed_blocks {
            let confirmed_block = rpc_client
                .get_block(confimed_blockn)
                .await
                .context("get confirmed blocks")?;

            let leader_pubkeys = confirmed_block.rewards.iter().map(|r| r.pubkey).collect();
            
        }

        if end_slot > latest_slot {
            break;
        }
    }

    // let block = rpc_client.get_block(0).await.context("get block")?;

    // eprintln!("Block: {:?}", block);

    // let listener = TcpListener::bind("127.0.0.1:2345")
    //     .await
    //     .context("binding")?;
    // while let Ok(stream) = listener.accept().await {}

    // eprintln!("Block: {:?}", block);

    Ok(())
}
