use std::fmt;

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
}
