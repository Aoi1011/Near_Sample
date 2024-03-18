use std::sync::{Arc, Mutex};

use anyhow::Context;
use dotenv::dotenv;
use lantern::MerkleTree;
use rocksdb::DB;
use solana_client::nonblocking::rpc_client::RpcClient;

const ONE_EPOCH: u64 = 432_000;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let path = "./storage/testdb";
    let db = Arc::new(Mutex::new(DB::open_default(path).context("open DB")?));
    let url = std::env::var("RPC_URL").context("read RPC_URL")?;
    let rpc_client = Arc::new(RpcClient::new(url));

    let latest_slot = rpc_client.get_slot().await.expect("get slot");

    // let vote_accounts = rpc_client
    //     .get_vote_accounts()
    //     .await
    //     .context("get vote accounts")?;

    // eprintln!("Vote accounts: {}", vote_accounts.current.len());

    let limit = latest_slot / ONE_EPOCH;
    let mut handles = Vec::new();
    for epoch in 0..limit {
        let start_slot = epoch * ONE_EPOCH;
        let end_slot = ((epoch + 1) * ONE_EPOCH) - 1;

        let db = db.clone();
        let rpc_client = rpc_client.clone();
        let handle = tokio::spawn(async move {
            let confirmed_blocks = rpc_client
                .get_blocks(start_slot, Some(end_slot))
                .await
                .expect("get confirmed blocks");

            println!(
                "Got Confirmed blocks, the length is {}",
                confirmed_blocks.len()
            );

            let mut leaders: Vec<String> = Vec::new();
            for confimed_blockn in confirmed_blocks {
                let confirmed_block = rpc_client
                    .get_block(confimed_blockn)
                    .await
                    .expect("get confirmed blocks");

                let leader_pubkeys: Vec<String> = confirmed_block
                    .rewards
                    .iter()
                    .map(|r| r.pubkey.clone())
                    .collect();
                leaders.extend(leader_pubkeys);
            }

            let tree = MerkleTree::new(leaders);

            println!("Epoch {epoch}: the root hash is {}", &tree.root.hash);

            db.lock()
                .unwrap()
                .put(epoch.to_be_bytes(), tree.root.hash)
                .expect("put info");
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
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
