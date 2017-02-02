extern crate hyper;
extern crate reqwest;
extern crate rustc_serialize;

use error::DomoError;
use hyper::Url;
use rustc_serialize::json::Json;

pub struct Client {
    json_url: String,
}

impl Client {
    pub fn new<S>(json_url: S) -> Client where S: Into<String> {
        Client { json_url: json_url.into() }
    }

    pub fn set_switch_value(&self, rid: u16, value: bool) -> Result<(), DomoError> {
        let cmd = if value { "On" } else { "Off" };
        let mut url = Url::parse(self.json_url.as_str()).unwrap();
        url.query_pairs_mut().
            append_pair("type", "command").
            append_pair("param", "switchlight").
            append_pair("switchcmd", cmd).
            append_pair("idx", &rid.to_string());

        let mut resp = reqwest::get(url)?;
        if !resp.status().is_success() {
            // FIXME: create a more specific error type
            return Err(DomoError::Domoticz);
        }

        let data = Json::from_reader(&mut resp).unwrap();
        let status = data.find("status").unwrap().as_string();
        if status != Some("OK") {
            return Err(DomoError::Domoticz);
        }

        Ok(())
    }

    pub fn get_switch_value(&self, rid: u16) -> Result<bool, DomoError> {
        let mut url = Url::parse(self.json_url.as_str()).unwrap();
        url.query_pairs_mut().
            append_pair("type", "devices").
            append_pair("rid", &rid.to_string());

        let mut resp = reqwest::get(url)?;
        if !resp.status().is_success() {
            // FIXME: create a more specific error type
            return Err(DomoError::Domoticz);
        }

        let data = Json::from_reader(&mut resp).unwrap();
        let status = data.find("status").unwrap().as_string();
        if status != Some("OK") {
            return Err(DomoError::Domoticz);
        }

        let results = data.find("result").unwrap().as_array().unwrap();
        if results.len() != 1 {
            return Err(DomoError::Domoticz);
        }

        match results[0].find("Status").unwrap().as_string() {
            Some("On") => { Ok(true) },
            Some("Off") => { Ok(false) },
            _ => { Err(DomoError::Domoticz) },
        }
    }
}
