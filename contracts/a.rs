#![allow(unused)]
use std::convert::TryInto;
use std::error::Error;
use web3::contract::{Contract, Options};
use web3::types::{Address, U256};
use web3::transports::Http;
use web3::Web3;

#[derive(Debug)]
struct Escrow {
    seller: Address,
    buyer: Address,
    arbitrator: Address,
    amount: U256,
    buyer_approved: bool,
    seller_approved: bool,
}

impl Escrow {
    fn new(seller: Address, buyer: Address, arbitrator: Address, amount: U256) -> Self {
        Escrow {
            seller,
            buyer,
            arbitrator,
            amount,
            buyer_approved: false,
            seller_approved: false,
        }
    }

    fn approve_by_buyer(&mut self) {
        self.buyer_approved = true;
    }

    fn approve_by_seller(&mut self) {
        self.seller_approved = true;
    }

    fn release_to_buyer(&self, contract: &Contract<Web3<Http>>) -> Result<(), Box<dyn Error>> {
        if self.buyer_approved && self.seller_approved {
            let result = contract.call(
                "releaseToBuyer",
                (self.amount,),
                Options::default().gas_limit(300000u64),
            ).wait()?;
            Ok(())
        } else {
            Err("Release not authorized")?
        }
    }

    fn release_to_seller(&self, contract: &Contract<Web3<Http>>) -> Result<(), Box<dyn Error>> {
        if !self.buyer_approved && self.seller_approved {
            let result = contract.call(
                "releaseToSeller",
                (self.amount,),
                Options::default().gas_limit(300000u64),
            ).wait()?;
            Ok(())
        } else {
            Err("Release not authorized")?
        }
    }

    fn dispute(&self, contract: &Contract<Web3<Http>>) -> Result<(), Box<dyn Error>> {
        let result = contract.call(
            "dispute",
            (),
            Options::default().gas_limit(300000u64),
        ).wait()?;
        Ok(())
    }

    fn get_balance(&self, contract: &Contract<Web3<Http>>) -> Result<U256, Box<dyn Error>> {
        let balance: U256 = contract.query(
            "getBalance",
            (),
            None,
            Options::default(),
            None,
        ).wait()?;
        Ok(balance)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the Ethereum network
    let transport = Http::new("http://localhost:8545")?;
    let web3 = Web3::new(transport);

    // Define the contract ABI
    let contract_abi = json::parse(r#"[{"inputs":[{"internalType":"address payable","name":"_seller","type":"address"},{"internalType":"address payable","name":"_buyer","type":"address"},{"internalType":"address","name":"_arbitrator","type":"address"}],"stateMutability":"payable","type":"constructor"},{"inputs":[],"name":"approveByBuyer","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"approveBySeller","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"dispute","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"getBalance","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMut
