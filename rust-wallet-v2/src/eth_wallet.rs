/*
PROJECT SPECS:
* 1) Make function for generating a key pair (public/private)
* 2) Take the Private Key and convert to Hex address and store/save it
* 3) Share and hold the wallet balance
*/

//Running for error handeling
use crate::utils;
use anyhow::Result;

use secp256k1::{
    rand::{rngs},
    PublicKey, SecretKey,
};

//Scoping in new objects for reading and writing files
use serde::{Deserialize, Serialize};
use std::io::BufWriter;
use std::str::FromStr;
use std::{fs::OpenOptions, io::BufReader};

//function for getting the users address using the keccak packages allowing for transfers
use tiny_keccak::keccak256;
use web3::{
    transports,
    types::{Address, TransactionParameters, H256, U256},
    Web3,
};


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
    let mut rng = rngs::JitterRng::new_with_timer(utils::get_nstime);
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
    @returns: Result<()> -> disk containing the information we need
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
    /*
    Function: from_fiel
    Purpose: Loads up a wallet
    @Params: file_path -> the location pulling the information from
    @returns: Result<()> -> disk containing the information we need
    @notes: N/A
    */
    pub fn from_file(file_path: &str) -> Result<Wallet> {
        let file = OpenOptions::new().read(true).open(file_path)?;
        let buf_reader = BufReader::new(file);
        let wallet: Wallet = serde_json::from_reader(buf_reader)?;
        Ok(wallet)
    }
    /*
    Function: get_secret_key
    Purpose: Retreives the secret key
    @Params: &Self -> the wallet itslef
    @returns: Result<()> -> disk containing the information we need
    @notes: N/A
    */
    pub fn get_secret_key(&self) -> Result<SecretKey> {
        let secret_key = SecretKey::from_str(&self.secret_key)?;
        Ok(secret_key)
    }
    /*
    Function: get_public_key
    Purpose: Retreives the public key
    @Params: &Self -> the wallet itslef
    @returns: Result<()> -> disk containing the information we need
    @notes: N/A
    */
    pub fn get_public_key(&self) -> Result<PublicKey> {
        let secret_key = PublicKey::from_str(&self.public_key)?;
        Ok(secret_key)
    }
    
    /*
    Function: get_balance
    Purpose: Converts current balance into eth
    @Params: &Self -> the wallet itslef
             &Web3<transports::WebSocket> -> Web3 API/Socket interactor
    @returns: Result<(f64)> -> current balance of the wallet
    @notes: N/A
    */
    pub async fn get_balance(&self, web3_connection: &Web3<transports::WebSocket>) -> Result<U256> {
        let wallet_address = Address::from_str(&self.public_address)?;
        let balance = web3_connection.eth().balance(wallet_address, None).await?;

        Ok(balance)
    }

    /*
    Function: get_balance_in_eth
    Purpose: Converts current balance into eth
    @Params: &Self -> the wallet itslef
             &Web3<transports::WebSocket> -> Web3 API/Socket interactor
    @returns: Result<(f64)> -> current balance of the wallet in eth from wei
    @notes: N/A
    */
    pub async fn get_balance_in_eth(
        &self,
        web3_connection: &Web3<transports::WebSocket>,
    ) -> Result<f64> {
        let wei_balance = self.get_balance(web3_connection).await?;
        Ok(utils::wei_to_eth(wei_balance))
    }   
}


/*
 Function: establish_web3_connection
 Purpose: connects the rust project to the blockchain via websocket connection
 @Params: url: -> websocket connector
 @returns: Result<Web3<transports::WebSocket>> -> connected state
 @notes: N/A
 */
pub async fn establish_web3_connection(url: &str) -> Result<Web3<transports::WebSocket>> {
    let transport = transports::WebSocket::new(url).await?;
    Ok(Web3::new(transport))
}

/*
 Function: return_public_address
 Purpose: creaets template/ground 0 for the eth transactions
 @Params: to: -> the locationto where it is going (address)
          eth_value -> amount of eth that is going through
 @returns: TransactionParameters -> groundwork for the eth transaction to be used in other functions
 @notes: N/A
 */
pub fn create_eth_transaction(to: Address, eth_value: f64) -> TransactionParameters {
    TransactionParameters {
        to: Some(to),
        value: utils::eth_to_wei(eth_value),
        ..Default::default()
    }
}

/*
 Function: sign_and_send
 Purpose: packages up the transaction and sends it across chain
 @Params: web3: &Web3<transports::WebSocket> -> endPoint API connector
          SecretKey-> the useres public key
          transaction: TransactionParameters, -> the transaction that currently happened
 @returns: Address -> the address of the users wallet
 @notes: We are using hashing and the keccak256 package to get the actual address from the
        serialized publicKey
 */
pub async fn sign_and_send(
    web3: &Web3<transports::WebSocket>,
    transaction: TransactionParameters,
    secret_key: &SecretKey,
) -> Result<H256> {
    let signed = web3
        .accounts()
        .sign_transaction(transaction, secret_key)
        .await?;

    let transaction_result = web3
        .eth()
        .send_raw_transaction(signed.raw_transaction)
        .await?;
    Ok(transaction_result)
}
