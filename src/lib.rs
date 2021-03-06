extern crate hyper;
extern crate lettre;
extern crate reqwest;
extern crate rustc_serialize;

pub mod config;
pub mod domoticz;
pub mod error;
pub mod message;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
