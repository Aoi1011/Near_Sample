# What is Ethereum?


## Compared to Bitcoin
Ethereum shares many common elements with other open blockchains: a peer to peer network connecting participants, a Byzantine fault-tolerant consensus algorithm for synchronization of state updates, the use of cryptographic primitives such as digital signatures and hashes, and digital curency. 

Ethereum's purpose is not primarily to be a digital currency ayment network. While the digital currency ether is both integral to and necessary for the operation of Ethereum, ether is intended as a utility currency to pay for use of the Ethreum platform as the world computer. 

Ethereum is designed to be a general-purpose programmable blockchain that runs a virtual machine capable of executing code of arbitrary and unbounded complexity. Ethereum's language is Turing complete, meaning that Ethereum can straightforwardly function as a general-purpose computer. 


## Components of a Blockchain
- A P2P network connecting participants and propagating txs and blocks of verified txs, based on a standardized "gossip" protocol. 
- Messages, in the form of txs, representing state transitions
- A set of consensus rules, governing what constitutes a tx and what makes for a valid state transition
- A state machine that processes txs according to the consensus rules
- A chain of cryptographically secured blocks that acts as a journal of all the verified and accepted state transitions
- A consensus algorithm that decentralizes control over the blockchain, by forcing participants to cooperate in the enforcement of the consensus rules
- Incentivization scheme to secure the state machine in an open environment
- Open source (clients)

open, public, global, decentralized, neutral, and censorship-resistant


## The Birth of Ethereum
Developers faced a conundrum: they either needed to build on top of Bitcoin or start a new blockchain. 

Could support a broad variety of applications by being programmed. 


## Ethereum's Four Stages of Development
"hard forks" - change functionality in a way that is not backward compatible


## Ethereum: A General-Purpose Blockchain
Ethereum is also a distributed state machine. Ethereum tracks the state transitions of a general-purpose data store, a store that can hold any data expressible as a *key-value* tuple. 
Ethereum has memory that stores both code and data, and it uses the Ethereum blockchain to track how this memory changes into its state machine and run that code, storing the resulting state changes in its blockchain. 


## Ethereum's Components
- P2P network
Ethereum runs on the Ethereum main network, which is addressable on TCP port 20202, and runs a protocol called DEVp2p

- Consensus rules

- Transactions
Ethereum txs are network messages that include a sender, recipient, value, and data payload

- State machine
Ethereum state transitions are processed by the Ethereum Virtual Machine(EVM), a stack-based virtual machine that executes bytecode (machine-language instructions). EVM programs, called "smart contracts"

- Data structures
Ethereum's state is stored locally on each node as a database (usually Google's LevelDB),  which contains the txs and system state in a serialized hashed data structure called a Merkle Patrica Tree. 

- Consensus algorithm
POS

- Economic security

- Clients
Client software, Go-Ethereum and Parity


## Ethereum and Turing Completeness
Ethereum's ability to execute a stored program, in a state machine called the Ethereum Virtual Machine, while reading and writing data to memory makes it a Turing-compelte system adn therefore a UTM(Universal Turing Machine). 

### Turing Completeness as a "Feature"
Turing completeness is very dangerous, particularly in open access systems like public blockchain, because of the halting problem we touched on earlier. 

### Implications of Turing Completeness
As Turing proved, Ethereum can't predict if a smart contract will terminate, or how long it will run, without actually running it (possibly running forever). 

Ethereum introduces a metering mechanism called gas. As the EVM executes a smart contract, it carefully accounts for every instruction(computation, data access, etc.). 


## From General-Purpose Blockchains to Decentralized Applications
Ethereum started as a way to make a general-purpose blockchain that could be programmed for a variety of uses. 

A DApp is composed of at least:
- A smart contracts on a blockchain
- A web frontend user interface

In addition, many DApps include other decentralized components, such as:
- A decentralized (P2P) storage protocol and platform
- A decentralized (P2P) messaging protocol and platform

## Ethereum's Development Culture
The community's development culture is focused on the future rather than the past. The mantra is "move fast and break things."

## Why Learn Ethereum?

