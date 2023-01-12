use bitcoincore_rpc::{Auth, Client, RpcApi};

fn main() {
    let rpc = Client::new(
        "http://127.0.0.1:18332",
        Auth::UserPass("alice".to_string(), "alicepassword".to_string()),
    )
    .unwrap();

    let best_block_hash = rpc.get_best_block_hash().unwrap();
    println!("best block hash: {best_block_hash}");
}
