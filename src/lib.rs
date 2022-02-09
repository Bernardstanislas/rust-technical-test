use std::fmt;
use std::fs::File;
use std::io::{Write, Read, Error, Seek, SeekFrom};

#[derive(Debug)]
struct Bounty {
    title: String,
    description: String,
    amount: u32,
    payment_token_address: String
}

impl fmt::Display for Bounty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} [{} at {}]: {}", self.title, self.amount, self.payment_token_address, self.description)
    }
}

trait BountyStore {
    fn save_bounty(&mut self, bounty: Bounty) -> Result<(), Error>;
}

struct FileBountyStore<'a> {
    store_file: &'a File
}

impl BountyStore for FileBountyStore<'_> {
    fn save_bounty(&mut self, bounty: Bounty) -> Result<(), Error> {
        write!(self.store_file, "{}", bounty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounty_creation() {
        let bounty = Bounty {
            title: "Bounty platform".to_string(),
            description: "Develop a bounty platform".to_string(),
            amount: 100,
            payment_token_address: "0x0000000000000000000000000000000000000000".to_string()
        };

        assert_eq!(bounty.title, "Bounty platform");
        assert_eq!(bounty.description, "Develop a bounty platform");
        assert_eq!(bounty.amount, 100);
        assert_eq!(bounty.payment_token_address, "0x0000000000000000000000000000000000000000");
    }

    #[test]
    fn test_bounty_display() {
        let bounty = Bounty {
            title: "Bounty platform".to_string(),
            description: "Develop a bounty platform".to_string(),
            amount: 100,
            payment_token_address: "0x0000000000000000000000000000000000000000".to_string()
        };

        assert_eq!(format!("{}", bounty), "Bounty platform [100 at 0x0000000000000000000000000000000000000000]: Develop a bounty platform");
    }

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
