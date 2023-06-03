/*
PROJECT SPECS:
* 1) Make function for generating a key pair (public/private)
* 2) Take the Private Key and convert to Hex address and store/save it
* 3)
*/

//Running for error handeling
use anyhow::{bail, Result};

use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};

//Scoping in new objects for reading and writing files
use serde::{Deserialize, Serialize};
use std::io::BufWriter;
use std::str::FromStr;
use std::{fs::OpenOptions, io::BufReader};

//function for getting the users address using the keccak packages allowing for transfers
use tiny_keccak::keccak256;
use web3::{types::Address};

/*
 Function: generate_keypair
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

pub fn public_key_address(public_key: &PublicKey) -> Address {
    let public_key= public_key.serialize_uncompressed();
    //check if serialize returns the correct thing
    debug_assert_eq!(public_key[0], 0x04);
    let hash = keccak256(&public_key[1..]); //Using hashing to parse the public key

    Address::from_slice(&hash[12..]) //setting value of return var
}

//Making the wallet structure
#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    //Members of class wallet
    //Public
    pub secret_key: String,
    pub public_key: String,
    pub public_address: String,
    //Private
}


impl Wallet {
    /*
    Function: new
    Purpose: Default Ctor for the wallet object, saving data to file,
            and loading data from file (wallet)
    @Params: secret_key -> Uses secret key;
             public_key -> Users public key
    @returns: Self -> constructed wallet (this)
    @notes: N/A
     */
    pub fn new(secret_key: &SecretKey, public_key: &PublicKey) -> Self {
        let addr: Address = public_key_address(&public_key);
        Wallet {
            secret_key: secret_key.to_string(),
            public_key: public_key.to_string(),
            public_address: format!("{:?}", addr), //Formatting the address variable
        }
    }
    /*
    Function: save_as_file
    Purpose: saves the data from the wallet to a disk
    @Params: &self -> the wallet itslef
            file_path -> string for where the resulting file is going to go
    @returns: Disk<()> -> disk containing the information we need
    @notes: We are using hashing and the keccak256 package to get the actual address from the
            serialized publicKey
    */
    pub fn save_as_file(&self, file_path: &str) -> Result<()> {
        //Using Standard Library to create new object
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;
        let buf_writer = BufWriter::new(file);
        serde_json::to_writer_pretty(buf_writer, self)?;
        Ok(())
    }
}

