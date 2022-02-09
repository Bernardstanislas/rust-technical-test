# Rust technical test

This small Rust project is a CLI tool used to store bounty information.
It currently only supports local filesystem storage but could be extended to other storage mechanisms such as IPFS.

## Usage

Simply run `cargo run` to run the dev version and the program will prompt for bounty information.

## Todo

- Clean error handling, most errors are propagated up top to the `main` function and will display ugly in a console
- Add support for IPFS by implementing an IPFS version of the `BountyStore` trait
- Clean up the modules layout to follow a more "Rustacian" approach, and remove the ugly hack `use crate::store::BountyStore;`
- Take environment variables into account in production environments — currently only working for dev versions
- Write inline documentation to make `cargo doc` more interesting
- Put a loggin mechanism in place with different levels of verbosity
