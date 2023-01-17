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

Contracts have addresses, just like EOAs. Contracts can also send and receive ether, just like EOAs. However, when a tx destination is a contract address, it causes that contract to run in the EVM, using the transaction, and the transaction's data, as its input. 

A contract account does not have a private key, it cannot initiate a transaction. Only EOAs can initiate txs, but contracts can react to transactions by calling other contracts, building complex execution paths. 

## A simple contract: A test ehter faucet

```solidity
// our first contract is a faucet!
contract Faucet {
	function withdraw(uint withdraw_amount) public {
		// Limit withdrawal amount
		require(withdraw_amount <= 10000000000000);

		// Send the amount to the address that requested it
		msg.sender.transfer(withdraw_amount);
	}

	// Accept any incoming amount
	function() public payable {}
}
```

Comments are for humans to read and are not included in the executable EVM byte-code. 


## Compiling the Faucet Contract
We need to use a Solidity compiler to convert the Solidity code into EVM bytecode so it can be executed by the EVM on the blockchain itself.


## Creating the Contract on the Blockchain
contact -> compile -> bytecode -> register the contract on the Ethereum blockchain.


## Interacting with the Contract

### Viewing the Contract Address in a Block Explorer

