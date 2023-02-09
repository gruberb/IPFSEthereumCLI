use std::{sync::Arc, time::Duration};

use ethers::{
	middleware::SignerMiddleware,
	prelude::LocalWallet,
	providers::{Http, Provider},
	signers::{Signer, Wallet},
	utils::Anvil,
};

/// Possible NetworkErrors
pub enum NetworkError {
	/// No connection could be established to the network (mainnet/testnet/local)
	NoConnection,
}

/// Network environments the CLI can target (possibly mainnets and testnets as well as local ones)
pub enum Environment {
	/// The local testing environment ANVIL
	Anvil,
}

/// Parameters needed for a succesfull network connection
#[allow(dead_code)]
pub struct Network {}

/// A shortcut for the verbose ethers-rs Middleware type
#[allow(dead_code)]
pub type EthersClient =
	SignerMiddleware<Provider<Http>, Wallet<ethers::core::k256::ecdsa::SigningKey>>;

impl Network {
	#[allow(dead_code)]
	pub(crate) fn connect(environment: Environment) -> Result<Arc<EthersClient>, NetworkError> {
		match environment {
			Environment::Anvil => {
				let anvil = Anvil::new().spawn();
				let wallet: LocalWallet = anvil.keys()[0].clone().into();

				let provider = Provider::<Http>::try_from(anvil.endpoint())
					.map_err(|_| NetworkError::NoConnection)?
					.interval(Duration::from_millis(10u64));

				// Instantiate the client with the wallet
				let client =
					SignerMiddleware::new(provider, wallet.with_chain_id(anvil.chain_id()));
				Ok(Arc::new(client))
			}
		}
	}
}
