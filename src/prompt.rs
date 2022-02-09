use std::io::{BufRead, Write, Error};
use crate::bounty::Bounty;

pub struct BountyPrompt<R, W> {
    pub reader: R,
    pub writer: W,
}

impl<R, W> BountyPrompt<R, W>
where
    R: BufRead,
    W: Write
{
    pub fn new_bounty(&mut self) -> Result<Bounty, Error> {
        let title = self.ask_for("Bounty title: ")?;
        let description = self.ask_for("Bounty description: ")?;
        let amount = self.ask_for("Bounty amount: ")?.parse::<u32>().expect("Please enter a number");
        let payment_token_address = self.ask_for("Payment token address: ")?;
        Ok(Bounty {
            title,
            description,
            amount,
            payment_token_address
        })
    }

    fn ask_for(&mut self, question: &str) -> Result<String, Error> {
        write!(&mut self.writer, "{}", question)?;
        self.writer.flush()?;
        let mut answer = String::new();
        self.reader.read_line(&mut answer)?;
        Ok(answer.trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::BountyPrompt;

    #[test]
    fn test_bounty_prompt() {
        let input = b"Bounty platform\nDevelop a bounty platform\n100\n0x0000000000000000000000000000000000000000\n";
        let mut output = Vec::new();

        let mut bounty_prompt = BountyPrompt {
            reader: &input[..],
            writer: &mut output
        };

        let bounty = bounty_prompt.new_bounty().unwrap();

        assert_eq!(bounty.title, "Bounty platform");
        assert_eq!(bounty.description, "Develop a bounty platform");
        assert_eq!(bounty.amount, 100);
        assert_eq!(bounty.payment_token_address, "0x0000000000000000000000000000000000000000");
    }
}
