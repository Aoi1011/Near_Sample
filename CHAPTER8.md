# The Bitcoin Network

## Peer-to-Peer Network Architecture
Bitcoin is structured as a peer-to-peer network architecture on top of the internet. 

The term "bitcoin network" refers to the collection of nodes running the bitcoin P2P protocol. 

## Node Types and Roles
A bitcoin node is a collection of functions: routing, the blockchain database, mining, and wallet services. 

1. Full nodes
Maintain a complete and up-to-date copy of the blockchain. 

2. Simplified payment verification (SPV) nodes, light weight nodes
Maintain only a subset of the blockchain and verify txs using a method called SPV. They do not have a full copy of the blockchain. 

3. Mining nodes
Compete to create new blocks by running specialized hardware to solve the Proof-of-Work algorithm. Some mining nodes are also full nodes, maintaining a full copy of the blockchain, while others are light weight nodes participating in pool mining and depending on a pool server to maintain a full node. Miner

4. Wallet
User wallets might be part of a full node, as is usually the case with desktop bitcoin clients. 

## The Extended Bitcoin Network
The main bitcoin network, running the bitcoin P2P protocol, consists of between 5000 and 8000 listening nodes running variouts versions of the bitcoin reference client (Bitcoin Core) and a few hundred nodes running various other implementations of the bitcoin P2P protocol, such as Bitcoin Classic...

The extended bitcoin network includes the network running the bitcoin P2P protocol, described earlier, as well as nodes running specialized protocols. 

![Different types of nodes on the extended bitcoin network](https://learn.saylor.org/pluginfile.php/3452313/mod_book/chapter/18897/image.png)

![The extended bitcoin network showing various node types, gateways, and protocols](https://learn.saylor.org/pluginfile.php/3452313/mod_book/chapter/18897/image%20%281%29.png)

## Bitcoin Relay Networks
A *Bitcoin Relay Network* is a network that attempts to minimize the latency in the transmission of blocks between miners. The network consisted of several specialized nodes hosted on the AWS infrastructure around the world and served to connect the majority of miners and mining pools.

Relay networks are not replacements for bitcoin's P2P network. Instead they are overlay networks that provide additional connectivity between nodes with specialized needs. 

## Network Discovery
When a new node boots up, it must discover other bitcoin nodes on the network in order to participate. To start this process, a new node must discover at least one existing node on the network and connect to it. 

## Full Nodes
Full nodes are nodes that maintain a full blockchain with all txs. They should be called "full blockchain nodes." 

Full blockchain nodes maintain a compelte and up-to-date copy of the bitcoin blockchain with all the txs, which they independently build and verify, starting with the very first block and building up to the latest known block in the network. 

## Exchanging "Inventory"
The first thing a full node will do once it connects to peers is try to construct a complete blockchain. 

## Simplified Payment Verification (SPV) Nodes
Not all nodes have the ability to store the full blockchain. Many bitcoin clients are designed to run on space-and power-constrained devides, such as smartphones, tablets, or embedded systems. 

SPV nodes download only the block headers and do not download the txs included in each block. The resulting chain of blocks, without txs, is 1000 times smaller than the full blockchain. 

SPV verifies txs by reference to their depth in the blockchain instead of their height. 

An SPV node cannot validate whether the UTXO is unspent. Instead, the SPV node will establish a link between the transaction and the block that contains it, using a merkle path. 

An SPV node cannot be persuaded that a transaction exists in a block when  the tx does not in fact exist. The SPV node establishes the existence of a tx in a block by requesting a merkle path proof and by validating the Proof-of-Work in the chain of blocks. 
