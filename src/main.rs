#[macro_use]
extern crate dotenv_codegen;

use std::fs;
use std::io;

mod store;
mod prompt;
mod bounty;
use crate::store::BountyStore;

fn main() {
    let stdio = io::stdin();
    let input = stdio.lock();

    let output = io::stdout();

    let store_file_path = dotenv!("BOUNTY_STORE_FILE_PATH");

    let store_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(store_file_path)
        .unwrap();

    let mut bounty_store = store::FileBountyStore{ store_file: &store_file };
    let mut bounty_prompt = prompt::BountyPrompt {
        reader: input,
        writer: output
    };

    let bounty = bounty_prompt.new_bounty().unwrap();
    bounty_store.save_bounty(bounty).unwrap();
}
