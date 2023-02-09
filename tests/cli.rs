use std::process::Command;

use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions // Run programs

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
	let mut cmd = Command::cargo_bin("ipfs-contract")?;

	cmd.arg("--file").arg("test/file/doesnt/exist");
	cmd.assert()
		.failure()
		.stderr(predicate::str::contains("No file found at given"));

	Ok(())
}

#[test]
fn contract_was_deployed() -> Result<(), Box<dyn std::error::Error>> {
	let mut cmd = Command::cargo_bin("ipfs-contract")?;

	cmd.arg("--file").arg("foo.txt");
	cmd.assert().success().stdout(predicate::str::contains(
		"0x5fbdb2315678afecb367f032d93f642f64180aa3",
	));

	Ok(())
}

#[test]
fn hash_value_set_in_contract() -> Result<(), Box<dyn std::error::Error>> {
	let mut cmd = Command::cargo_bin("ipfs-contract")?;

	cmd.arg("--file").arg("foo.txt");
	cmd.assert().success().stdout(predicate::str::contains(
		"QmW3J3czdUzxRaaN31Gtu5T1U5br3t631b8AHdvxHdsHWg",
	));

	Ok(())
}
