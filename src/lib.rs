#[derive(Debug)]
struct Bounty {
    title: String,
    description: String,
    amount: u32,
    payment_token_address: String
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
}
