use std::{
	fmt::{Debug, Display, Formatter},
	fs::File,
};

use async_trait::async_trait;
use ipfs_api_backend_hyper::{Error, IpfsApi, IpfsClient};

/// Possible errors when connection to our storage environments
#[derive(Debug)]
pub enum StorageError {
	/// Could not connect to IPFS, possibly daemon is not running
	IPFSError(Error),
}

impl Display for StorageError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			StorageError::IPFSError(_) => write!(
				f,
				"Could not save to IPFS! Please make sure the ipfs daemon is running."
			),
		}
	}
}

impl std::error::Error for StorageError {}

/// The has of the file we want to store
pub struct Hash(String);

impl Display for Hash {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

/// Each valid storage option has to implement the following functions to be able to work
/// throughout this application
#[async_trait(?Send)]
pub trait StorageFaciliation {
	async fn save(provider: StorageProvider, file: File) -> Result<Hash, StorageError>;
	// async fn load(provider: StorageProvider, hash: Hash) -> File;
}

/// Possible storage providers
pub enum StorageProvider {
	/// Sending files to IPFS
	Ipfs,
	// LOCAL,
}

pub struct Storage {}

#[async_trait(?Send)]
impl StorageFaciliation for Storage {
	async fn save(provider: StorageProvider, file: File) -> Result<Hash, StorageError> {
		match provider {
			StorageProvider::Ipfs => {
				let client = IpfsClient::default();
				match client.add(file).await {
					Ok(response) => Ok(Hash(response.hash)),
					Err(err) => Err(StorageError::IPFSError(err)),
				}
			} // StorageProvider::LOCAL => {
			  //     let mut hasher = Sha256::new();
			  //     let _ = io::copy(&mut file, &mut hasher)?;
			  //     let hash = hasher.result();
			  //     let mut output = std::fs::File::create(checksum_filename)?;
			  //
			  //     Ok(Hash(hash))
			  // }
		}
	}

	// fn load(provider: StorageProvider, hash: Hash) -> File {
	//     todo!()
	// }
}
