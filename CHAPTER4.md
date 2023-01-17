# Cryptography
Cryptography means "secret writing" in Greek, 
Cryptography can, for example, also be used to prove knowledge of a secret without revealing that secret, or to prove the authenticity of data. 



## Keys and Addresses
As we saw earlier in the book, Ethereum has 2 different types of accounts: 
1. externally owned accounts (EOAs)
2. contracts

Ownership of ether by EOAs is established through digital private keys, Ethereum addresses, and digital signatures. The private keys are at the heart of all user interaction with Ethereum. In fact, account addresses are derived directly from private keys: a private key uniquiely determines a signle Ethereum address, also known as an account.

Access and control of funds is achieved with digital signatures, which are also created using the private key. 

In public key cryptography-based systems, such as that used by Ethereum, keys come in paris consisting of a private key and public key. 


## Public Key Cryptography and Cryptocurrency
Public key cryptography (also called "asymmetric cryptography") is a core part of modern-day information security.

Public key cryptography uses unique keys to secure information. These keys are based on mathematical functions that have a special property: it is easy to calculate them, but hard to calculate their inverse. 

[Cryptography](https://en.wikipedia.org/wiki/Cryptography)
[Trapdoor function](https://en.wikipedia.org/wiki/Trapdoor_function)
[Integer factorization](https://en.wikipedia.org/wiki/Integer_factorization)
[Discrete logarithm](https://en.wikipedia.org/wiki/Discrete_logarithm)
[Elliptic-curve cryptography](https://en.wikipedia.org/wiki/Elliptic-curve_cryptography)

A digital signature can be created to sign any message. For Ethereum txs, the details of the tx itself are used as the message. 


## Private Keys
A private key is simply a number, picked at random. Ownership and control of the private keys is the root of use control over all funds associated with the corresponding Ethereum address, as well as access to contracts that authorize that address. 
The private keys is used to create signatures required to spend to ether by proving ownership of funds used in a tx. 

The Ethereum private key is just a number. One way to pick your private keys randomly is to simply use a coin, pencil, and paper: 
toss a coin 256 times and you have the binary digits of a random private key you can use in an Ethereum wallet. 

### Generating a private key from a random number
The first and most important step in generating keys is to find a secure source of entropy, or randomness. 

Note that the private key generation process is an offline one; it does not require any communication with the Ethereum network, or indeed any communication with anyone at all. As such, in order to pick a number that no one else will ever pick, it needs to be truly random. 

f8f8a2f43c8376ccb0871305060d7b27b0554d2cc72bccf41b2705608452f315

hexadecimal format (256 bits shown as 64 hexadecimal digits)


## Public Keys
An Ethereum public key is a *point* on an elliptic curve, meaning it is a set of x and y coordinates that satisfy the elliptic curve equation.

In simpler terms, an Ethereum public key is two numbers, joined together. These numbers are produced from the private key by a calculation that can only go one way. 

The public key is calculated  from the private key using elliptic curve multiplication, which is practically irreversible: K = k * G
where k is the private key, G is a constant point called the generator point, K is the resulting public key, and * is the special elliptic curve "multiplication" operator. 

### Elliptic Curve Cryptography Explained
Elliptic curve cryptography is a type of asymmetric or public key cryptography based on the discrete logarithm problem as expressed by addition and multiplication on the points of an elliptic curve

Ethereum uses a specific elliptic curve and set of mathematical constants, as defined in a standard called secp256k1, 

y ^ 2 = (x ^ 3 + 7) over (Fp)

y ^ 2 mod p = (x ^ 3 + 7) mod p

### Generating a Public Key
Starting with a private key in the form of a randomly generated number k, we multiply it by a predetermined point on the curve called the generator point G to produce another point somewhere else on the curve, which is the corresponding public key K:

K = k * G


## Cryptographic Hash Functions
Cryptographic hash functions are used throughout Ethereum. 

We address hash functions here because they are part of the transformation of Ethereum public key into addresses. 

In simple terms, a hash function is "any function that can be used to map data of arbitrary size to data of fixed size."

A cryptographic hash function is a one-way hash function that maps data of arbitrary size to a fixed-size string of bits. 

### Ethereum's Cryptographic Hash Function: Keccak-256
Ethereum uses the Keccak-256 cryptographic hash function in many places. Keccak was the winning algorithm, which became standardized. 

### Which Hash Function Am I Using?


## Ethereum Addresses
Ethereum addresses are unique identifiers that are derived from public keys or contracts using the Keccak-256 one-way hash function. 

Most often you will see Ethereum addresses with the prefix 0x that indicates they are hexadecimal-encoded, like this:
0x001d3f1ef827552ae1114027bd3ecf1f086ba0f9


### Ethereum Address Formats
Ethereum addresses are hexadecimal numbers, identifiers derived from the last 20 bytes of the Keccak-256 hash of the public key. 

### Inter Exchange Client Address Protocol

### Hex Encoding with Checksum in Capitalization (EIP-55)


