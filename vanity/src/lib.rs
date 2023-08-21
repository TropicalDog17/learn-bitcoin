use address::random_address;
use bitcoin::{Address, Network, PrivateKey, PublicKey};
pub fn vanity_address_miner(pattern: &str) {
    loop {
        let generated_address = random_address().to_string();
        if generated_address
            .to_lowercase()
            .starts_with(pattern.to_lowercase().as_str())
        {
            println!("Found vanity address: {generated_address}");
            return;
        }
    }
}
#[macro_use]
extern crate time_test;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_letters_pattern_1_times() {
        time_test!();

        let pattern = "1tu";
        let number_of_runs = 100;
        for _ in 1..number_of_runs {
            vanity_address_miner(pattern);
        }
    }
}
