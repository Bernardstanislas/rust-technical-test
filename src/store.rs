use std::fs::File;
use std::io::{Write, Error};
use crate::bounty::Bounty;

pub trait BountyStore {
    fn save_bounty(&mut self, bounty: Bounty) -> Result<(), Error>;
}

pub struct FileBountyStore<'a> {
    pub store_file: &'a File
}

impl BountyStore for FileBountyStore<'_> {
    fn save_bounty(&mut self, bounty: Bounty) -> Result<(), Error> {
        writeln!(self.store_file, "{}", bounty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Seek, SeekFrom};

    #[test]
    fn test_bounty_file_store() {
        let bounty = Bounty {
            title: "Bounty platform".to_string(),
            description: "Develop a bounty platform".to_string(),
            amount: 100,
            payment_token_address: "0x0000000000000000000000000000000000000000".to_string()
        };
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        let mut bounty_store = FileBountyStore { store_file: &tmpfile };

        bounty_store.save_bounty(bounty).unwrap();
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        let mut buffer = String::new();
        tmpfile.read_to_string(&mut buffer).unwrap();
        assert_eq!("Bounty platform [100 at 0x0000000000000000000000000000000000000000]: Develop a bounty platform", buffer);
    }
}
