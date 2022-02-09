use std::fs::File;
use std::io;

mod store;
mod prompt;
mod bounty;
use crate::store::BountyStore;

fn main() {
    let stdio = io::stdin();
    let input = stdio.lock();

    let output = io::stdout();

    let store_file = File::create("bounties.txt").unwrap();
    let mut bounty_store = store::FileBountyStore{ store_file: &store_file };
    let mut bounty_prompt = prompt::BountyPrompt {
        reader: input,
        writer: output
    };

    let bounty = bounty_prompt.new_bounty().unwrap();
    bounty_store.save_bounty(bounty).unwrap();
}
