# IPFS-Ethereum CLI 

### Pre-Requirements
1. Install `solc`
2. Install `ipfs`
3. Install `anvil`

### Run the following

```bash
$ ipfs daemon
$ anvil 
```

## Run the project

```bash
$ cargo run -- --file "PATH_TO_FILE"
```

or

```bash
$ cargo build
$ ./target/ipfs-contract --file "PATH_TO_FILE"
```

You can also get help via

```bash
$ cargo run -- -h

A CLI to upload a file to the IPFS network and deploy a smart contract 
with the resulting CID

Usage: ipfs-contract --file <FILE>

Options:
  -f, --file <FILE>  The path to the file you want to upload
  -h, --help         Print help
  -V, --version      Print version
```

## Example run

```bash
$ cargo run -- --file foo.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
    Running `target/debug/ipfs-contract --file foo.txt`
    
Address of the contract: 0x5fbdb2315678afecb367f032d93f642f64180aa3
Value of the contract: QmW3J3czdUzxRaaN31Gtu5T1U5br3t631b8AHdvxHdsHWg
```

## Run tests 

For the CLI tests, you have to have `ipfs` and `anvil` running in the background:

```bash
$ ipfs daemon
$ anvil 
```

```bash
$ cargo test 

running 1 test
test contract::tests::contract_compiles ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s

     Running tests/cli.rs (target/debug/deps/cli-981e64768c1317dd)

running 3 tests
test file_doesnt_exist ... ok
test contract_was_deployed ... ok
test hash_value_set_in_contract ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.55s
```
