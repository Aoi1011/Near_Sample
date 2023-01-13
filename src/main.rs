mod addr;
mod elliptic_curve;
mod key_to_address;

use addr::create_address;
use bitcoincore_rpc::{bitcoin::Txid, Auth, Client, RpcApi};
use key_to_address::key_to_address;

fn main() {
    key_to_address();
}

fn rpc_example() {
    let rpc = Client::new(
        "http://127.0.0.1:18332",
        Auth::UserPass("alice".to_string(), "alicepassword".to_string()),
    )
    .unwrap();

    let best_block_hash = rpc.get_blockchain_info().unwrap();
    println!("best block hash: {}", best_block_hash.blocks);
}

fn rpc_transaction() {
    let rpc = Client::new(
        "http://127.0.0.1:18332",
        Auth::UserPass("alice".to_string(), "alicepassword".to_string()),
    )
    .unwrap();
}
