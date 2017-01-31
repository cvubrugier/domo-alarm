extern crate rustc_serialize;
extern crate toml;

use error::DomoError;
use std::fs::File;
use std::io::Read;

#[derive(Debug, RustcDecodable)]
pub struct DomoConfig {
    pub email: EmailConfig,
    pub smtp: SmtpConfig,
    pub domoticz: DomoticzConfig,
    pub webapp: WebappConfig,
}

impl DomoConfig {
    pub fn new(path: &str) -> Result<DomoConfig, DomoError> {
        let mut file = try!(File::open(path));
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let domo_config: DomoConfig = toml::decode_str(content.as_str()).unwrap();

        Ok(domo_config)
    }
}

#[derive(Debug, RustcDecodable)]
pub struct EmailConfig {
    pub from: String,
    pub to: String,
}

#[derive(Debug, RustcDecodable)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

#[derive(Debug, RustcDecodable)]
pub struct DomoticzConfig {
    pub json_url: String,
    pub input_rid: u16,
    pub output_rid: u16,
}

#[derive(Debug, RustcDecodable)]
pub struct WebappConfig {
    pub socket_addr: String,
}
