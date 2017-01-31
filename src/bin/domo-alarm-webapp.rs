extern crate clap;
extern crate domo;
extern crate iron;
extern crate persistent;
extern crate router;
extern crate rustc_serialize;

use clap::{Arg, ArgMatches, App};
use domo::config::DomoConfig;
use domo::domoticz;
use domo::error::DomoError;
use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use iron::typemap::Key;
use persistent::Read;
use router::Router;
use rustc_serialize::json;

const INDEX_HTML: &'static [u8] = include_bytes!("../../assets/index.html");
const FAVICON_PNG: &'static [u8] = include_bytes!("../../assets/favicon-192x192.png");
const MANIFEST: &'static [u8] = include_bytes!("../../assets/manifest.webmanifest");

#[derive(RustcEncodable)]
struct AlarmResponse {
    is_ok: bool,
    message: String,
    alarm_on: bool,
}

struct Context {
    client: domoticz::Client,
    input_rid: u16,
    output_rid: u16,
}

impl Key for Context { type Value = Context; }

fn index_handler(_: &mut Request) -> IronResult<Response> {
    let content_type = "text/html".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, INDEX_HTML)))
}

fn favicon_handler(_: &mut Request) -> IronResult<Response> {
    let content_type = "image/png".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, FAVICON_PNG)))
}

fn manifest_handler(_: &mut Request) -> IronResult<Response> {
    let content_type = "application/manifest+json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, MANIFEST)))
}

fn version_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, format!("Version {}", domo::VERSION))))
}

fn alarm_request_parse(req: &Request) -> Option<bool> {
    const COMMAND_TAG: &'static str = "command=";

    if let Some(query) = req.url.query() {
        if query.starts_with(COMMAND_TAG) {
            let start = COMMAND_TAG.len();
            let param_value = &query[ start .. ];
            if param_value == "on" {
                return Some(true);
            } else if param_value == "off" {
                return Some(false);
            }
        }
    }

    return None;
}

fn alarm_on(context: &Context) -> AlarmResponse {
    if context.client.set_switch_value(context.input_rid, true).is_ok() {
        AlarmResponse {
            is_ok: true,
            message: "".to_string(),
            alarm_on: true,
        }
    } else {
        AlarmResponse {
            is_ok: false,
            message: "Cannot turn on the virtual switch. Please check Domoticz.".to_string(),
            alarm_on: false, /* Assume the virtual switch is off */
        }
    }
}

fn alarm_off(context: &Context) -> AlarmResponse {
    if context.client.set_switch_value(context.input_rid, false).is_err() {
        AlarmResponse {
            is_ok: false,
            message: "Cannot turn off the virtual switch. Please check Domoticz.".to_string(),
            alarm_on: false, /* Assume the virtual switch is off */
        }
    } else if context.client.set_switch_value(context.output_rid, false).is_err() {
        AlarmResponse {
            is_ok: false,
            message: "Cannot turn off the siren. Please check Domoticz.".to_string(),
            alarm_on: false, /* The virtual switch is off */
        }
    } else {
        AlarmResponse {
            is_ok: true,
            message: "".to_string(),
            alarm_on: false,
        }
    }
}

fn alarm_get(context: &Context) -> AlarmResponse {
    match context.client.get_switch_value(context.input_rid) {
        Ok(value) => {
            AlarmResponse {
                is_ok: true,
                message: "".to_string(),
                alarm_on: value,
            }
        },
        Err(_) => {
            AlarmResponse {
                is_ok: false,
                message: "Cannot get the virtual switch status. Please check Domoticz".to_string(),
                alarm_on: false, /* Assume the virtual switch is off */
            }
        }
    }
}

fn alarm_handler(req: &mut Request) -> IronResult<Response> {
    let context = req.get::<Read<Context>>().unwrap();
    let alarm_response = match alarm_request_parse(req) {
        Some(true) => { alarm_on(context.as_ref()) },
        Some(false) => { alarm_off(context.as_ref()) },
        None => { alarm_get(context.as_ref()) },
    };

    let encoded = json::encode(&alarm_response).unwrap();
    let content_type = "application/json".parse::<Mime>().unwrap();

    Ok(Response::with((content_type, status::Ok, encoded)))
}

fn run(matches: ArgMatches) -> Result<(), DomoError> {
    let config = DomoConfig::new(matches.value_of("config").unwrap())?;
    let context = Context {
        client: domoticz::Client::new(config.domoticz.json_url),
        input_rid: config.domoticz.input_rid,
        output_rid: config.domoticz.output_rid,
    };

    let mut router = Router::new();
    router.get("/", index_handler, "index");
    router.get("/favicon-192x192.png", favicon_handler, "favicon");
    router.get("/manifest.webmanifest", manifest_handler, "manifest");
    router.get("/version", version_handler, "version");
    router.get("/alarm.json", alarm_handler, "alarm");

    let mut chain = Chain::new(router);
    chain.link(Read::<Context>::both(context));
    Iron::new(chain).http(config.webapp.socket_addr.as_str()).unwrap();

    Ok(())
}

fn main() {
    let matches = App::new("domo-alarm-sender")
        .version(domo::VERSION)
        .author("Christophe Vu-Brugier")
        .about("Notification system for Domoticz to send an email and ring an alarm")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Sets the configuration file")
             .takes_value(true)
             .default_value("/etc/domo-alarm.conf"))
        .get_matches();

    if let Err(e) = run(matches) {
        println!("Error: {:?}", e);
    }

}
