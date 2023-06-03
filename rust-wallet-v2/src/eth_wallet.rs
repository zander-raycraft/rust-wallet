//Making a wallet on the ethereum network

/*
* 1) Make function for generating a key pair (public/private)
* 2)
*/


use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};

//function for getting the users address using the keccak packages allowing for transfers
use tiny_keccak::keccak256;
use web3::types::Address;

/*
 Function: generate_keypar
 Purpose: returns the address of the users publicKey
 @Params: PublicKey -> the useres public key
 @returns: Address -> the address of the users wallet
 @notes: We are using hashing and the keccak256 package to get the actual address from the
        serialized publicKey
 */
pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = secp256k1::Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(111);
    secp.generate_keypair(&mut rng)
}

/*
 Function: return_public_address
 Purpose: Generates a touple which will become the users private and public keys
 @Params: PublicKey -> the useres public key
 @returns: Address -> the address of the users wallet
 @notes: We are using hashing and the keccak256 package to get the actual address from the
        serialized publicKey
 */

pub fn return_public_address(public_key: &PublicKey) -> Address {
    let public_key= public_key.serialize_uncompressed();
    //check if serialize returns the correct thing
    debug_assert_eq!(public_key[0], 0x04);
    let hash = keccak256(&public_key[1..]); //Using hashing to parse the public key

    Address::from_slice(&hash[12..]) //setting value of return var
}