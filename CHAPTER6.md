# Transactions

## Transactions Outputs and Inputs

## Transaction Outputs
Transaction outputs consists of two parts:
1. An amount of bitcoin, denominated satoshis, the smallest bitcoin unit
2. A cryptographic puzzle that denominates the conditions required to spend the output

Transaction serialization - outputs
When transactions are transmitted over the network or exchanged between applications, they are serialized. 

## Transaction Inputs


## Transaction Scripts and Script Language
Most transactions processed through the bitcoin network have the form "Payment to Bob's bitcoin address" and are based on a script called a **Pay-to-Public-Key-Hash** script.

### Turing Incompleteness
The language is not *Turing Complete*, meaning that scripts have limited complexity and predictable execution times. 

### Stateless Verification
Bitcoin's tx script language is stateless, in that there is no state prior to execution of the script, or state saved after execution of the script. 

### Script Construction
Bitcoin's tx validation engine relies on two types of scripts to validate txs: a locking script and an unlocking script. 

A locking script is a spending condition placed on an output: it specifies the condition that must be met to spend the output in the future.

An unlocking script is a script that "solves", or satisfies, the conditions placed on an output by a locking script and allows the output to be spent. 

Every bitcoin validating node will validate txs by executing the locking and unlocking scripts together. Each input contains an unlocking script and refers to a previously existing UTXO. 

### Pay-to-Public-Key-Hash (P2PKH)
The vast majority of txs processed on the bitcoin network spend outputs locked with a Pay-to-Public-Key-Hash or "P2PKH" script. 
These outputs contain a locking script that locks the output to a public key hash, more commonly known as a bitcoin address. An output locked by a P2PKH script can be unlocked by presenting a public key and a digital signature created by the corresponding private key. 

For example, let's look at Alice's payment to Bob's Cafe again. Alice made a payment of 0.015 bitcoin to the cafe's bitcoin address. That transaction output would have a locking script of the form:
```
OP_DUP OP_HASH160 <Cafe Public Key Hash> OP_EQUALVERIFY OP_CHECKSIG
```

### Digital Signatures(ECDSA)
The digital signature algorithm used in bitcoin is the Elliptic Curve Digital Signature Algorithm. 
ECDSA is used by the script functions **OP_CHECKSIG**, **OP_CHECKSIGVERIFY**, **OP_CHECKMULTISIG**, and **OP_CHECKMULTISIGVERIFY**.

### How Digital Signatures Work
A digital signature is a mathematical scheme that consists of two parts. The first part is an algorithm for creating a signature, using a private key (the signing key), from a message (the tx). The second part is an algorithm that allows anyone to verify the signature, given also the message and a public key. 

### Bitcoin Addresses, Balances, and Other Abstractions

