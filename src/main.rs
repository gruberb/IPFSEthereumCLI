#![deny(clippy::all)]

use std::fs::File;

use clap::Parser;

mod contract;
mod network;
mod storage;

use crate::{
	contract::{Contract, ContractBuilder},
	network::Environment,
	storage::{Storage, StorageFaciliation, StorageProvider},
};

/// A CLI to upload a file to the IPFS network
/// and deploy a smart contract with the resulting CID
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
	/// The path to the file we want to upload
	#[arg(short, long)]
	file: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Parse the CLI arguments
	let cli = Cli::parse();

	// Read the file name/path from the CLI arg stored in `cli` and open the file
	let file = File::open(cli.file).expect("No file found at given path");

	// Pass the file onto our storage logic, where we decide where to upload or save it
	let hash = Storage::save(StorageProvider::Ipfs, file).await?;

	// We want to store the hash of the file in a smart contract
	// We choose ANVIL for local testing, but can integrate more testnets or mainnets
	let contract = Contract::deploy(
		Environment::Anvil,
		"SimpleStorage".to_string(),
		"./contracts".to_string(),
		hash.to_string(),
	)
	.await?;

	// Print the address of the contract
	println!("Address of the contract: {:?}", contract.address);
	// Print the value of the contract
	println!("Value of the contract: {}", contract.value);

	Ok(())
}
