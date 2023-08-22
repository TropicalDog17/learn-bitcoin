use crate::error::CustomError;
use address::random_address;
#[derive(Copy, Clone, Debug)]
pub struct VanityAddr;

pub enum VanityMode {
    Prefix,
    Suffix,
    Anywhere,
}
impl VanityAddr {
    pub fn generate(
        pattern: &str,
        case_sensitive: bool,
        vanity_mode: VanityMode,
    ) -> Result<String, CustomError> {
        if pattern.is_empty() {
            return Ok(random_address().to_string());
        }
        if pattern.len() >= 3 {
            return Err(CustomError("This library is not optimized enough to find 3 characters. Please consider using a smaller number!"));
        }
        let is_base58 = pattern
            .chars()
            .find(|c| c == &'0' || c == &'I' || c == &'O' || c == &'l');
        if is_base58.is_some() {
            return Err(CustomError("The input isn't in base58 format!, only contains alphanumeric character, exclude ['O', '0', 'I', 'l']"));
        }
        return Ok(vanity_address_miner(pattern, case_sensitive, vanity_mode));
    }
}
fn is_pattern_match(slice: &str, pattern: &str, case_sensitive: bool) -> bool {
    if case_sensitive {
        slice == pattern
    } else {
        slice.to_lowercase() == pattern.to_lowercase()
    }
}
fn vanity_address_miner(pattern: &str, case_sensitive: bool, vanity_mode: VanityMode) -> String {
    loop {
        let generated_address = random_address().to_string();
        let found = match vanity_mode {
            VanityMode::Prefix => {
                let slice = &generated_address[1..=pattern.len()];
                is_pattern_match(slice, pattern, case_sensitive)
            }
            VanityMode::Suffix => {
                let addr_len = generated_address.len();
                let slice = &generated_address[addr_len - pattern.len()..];
                is_pattern_match(slice, pattern, case_sensitive)
            }
            VanityMode::Anywhere => match case_sensitive {
                true => generated_address.contains(pattern),
                false => generated_address
                    .to_lowercase()
                    .contains(&pattern.to_lowercase()),
            },
        };

        if found {
            return generated_address;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_letters_pattern_1_times() {
        let pattern = "tu";
        let number_of_runs = 10;
        for _ in 1..number_of_runs {
            println!(
                "{}",
                vanity_address_miner(pattern, false, VanityMode::Anywhere)
            );
        }
    }
}
