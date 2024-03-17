use anyhow::Context;
use dotenv::dotenv;
use lantern::MerkleTree;
use solana_client::nonblocking::rpc_client::RpcClient;

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

    let limit = latest_slot / ONE_EPOCH;
    let mut roots = Vec::new();

    for epoch in 0..limit {
        let start_slot = epoch * ONE_EPOCH;
        let end_slot = ((epoch + 1) * ONE_EPOCH) - 1;

        let confirmed_blocks = rpc_client
            .get_blocks(start_slot, Some(end_slot))
            .await
            .context("get confirmed blocks")?;

        eprintln!(
            "Got Confirmed blocks, the length is {}",
            confirmed_blocks.len()
        );

        let mut leaders: Vec<String> = Vec::new();
        for confimed_blockn in confirmed_blocks {
            let confirmed_block = rpc_client
                .get_block(confimed_blockn)
                .await
                .context("get confirmed blocks")?;

            let leader_pubkeys: Vec<String> = confirmed_block
                .rewards
                .iter()
                .map(|r| r.pubkey.clone())
                .collect();
            leaders.extend(leader_pubkeys);
        }

        let tree = MerkleTree::new(leaders);
        roots.push(tree.root.hash);

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
