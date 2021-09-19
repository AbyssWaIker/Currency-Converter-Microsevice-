use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyConfig {
    pub api_key: String,
    pub host: String,
    pub port: String,
}

impl ::std::default::Default for MyConfig {
    fn default() -> Self { Self { host: "127.0.0.1".into(), port:"8080".into(), api_key: "".into() } }
}