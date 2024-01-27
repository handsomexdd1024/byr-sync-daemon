mod error;
pub use error::InvalidConfigError;

use std::convert::{From, TryFrom};
use serde::{Serialize,Deserialize};
use std::error::Error;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemConfig {
    pub name: String,
    pub upstream: String,
}

impl ItemConfig {
    pub fn new(name: String, upstream: String) -> ItemConfig {
        ItemConfig {
            name,
            upstream,
        }
    }
}

impl TryFrom<&String> for ItemConfig {
    type Error = serde_json::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
}
