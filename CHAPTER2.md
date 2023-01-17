# Ethereum Basics


## Ether Currency Units
Ethereum's currency unit is called ether, identified also as "ETH"
Ether is subdivided into smaller units, down to the smallest unit possible, which is named wei. 
One ether is 1 quintillion wei (1 * 10 ^ 18 or 1,000,000,000,000,000,000).


## Choosing an Ethereum Wallet
helps you manage your Ethereum account. In short, an Ethereum wallet is your gateway to the Ethereum system. It holds your keys and can create and broadcast txs on your behalf.


## Control and Responsibility
Open blockchains like are important because they operate as a decentralized system. control their own private keys, which are the things that control access to funds and smart contracts. 


## Introducing the World Computer
Ether is meant to be used to pay for running smart contracts, 

The EVM is a global singleton, meaning that it operates as if it were a global, single instance computer, running everywhere. Each node on the Ethereum network runs a local copy of the EVM to validate contract execution, while the Ethereum blockchain records the changing state of this wrold computer as it processes txs and smart contracts. 


## Externally Owned Accounts (EOAs) and Contracts
1. EOA (Externally Owned Accounts)
The type of account you created in the MetaMask wallet is called an externally owned account (EOA). 
Externally owned accounts are those that have a private key; having the private key means control over access to funds or contracts. 

2. contract account
A contract account has smart contract code, which a simple EOA can't have. A contract account does not have a private key. Instead, it is owned (and controlled) by the logic of its smart contract code: the software program recorded on the Ethereum blockchain at the contract account's creation and executed by the EVM. 


