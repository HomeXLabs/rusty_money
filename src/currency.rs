use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::include_str;
use std::str::FromStr;

static CURRENCY_JSON: &str = include_str!("../config/currencies.json");

lazy_static! {
    static ref CURRENCIES: HashMap<String, Currency> = serde_json::from_str(CURRENCY_JSON).unwrap();
}

/// The `Currency` type, which stores metadata about an ISO-4127 currency.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Currency {
    pub digit_separator: char,
    digit_separator_sequence: &'static str,
    pub exponent: u32,
    pub exponent_separator: char,
    pub iso_alpha_code: &'static str,
    pub iso_numeric_code: &'static str,
    pub symbol: &'static str,
    pub symbol_first: bool,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.iso_alpha_code)
    }
}

impl Currency {
    /// Finds a currency type given an alphabetic ISO-4217 currency code.
    pub fn find(name: String) -> Currency {
        match CURRENCIES.get(&name.to_lowercase()) {
            Some(c) => *c,
            None => panic!("{} is not a known currency", name), //TODO - more helpful message
        }
    }

    /// Returns a vector indicating where digit separators should be applied for a given currency.  
    pub fn digit_separator_sequence(self) -> Vec<usize> {
        let v: Vec<&str> = self.digit_separator_sequence.split(", ").collect();
        v.iter().map(|x| usize::from_str(x).unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn currency_known_can_be_found() {
        let c = Currency::find("USD".to_string());
        assert_eq!(c.iso_alpha_code, "USD");
        assert_eq!(c.exponent, 2);
    }

    #[test]
    #[should_panic]
    fn currency_unknown_raises_error() {
        Currency::find("fake".to_string());
    }
}
