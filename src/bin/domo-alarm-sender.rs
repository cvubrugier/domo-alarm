extern crate clap;
extern crate domo;

use clap::{Arg, ArgMatches, App};
use domo::config::DomoConfig;
use domo::error::DomoError;
use domo::domoticz;
use domo::message;

fn run(matches: ArgMatches) -> Result<(), DomoError> {
    let config = DomoConfig::new(matches.value_of("config").unwrap())?;
    let output_rid = config.domoticz.output_rid;
    let input_rid = config.domoticz.input_rid;

    let client = domoticz::Client::new(config.domoticz.json_url);

    let alarm_on = client.get_switch_value(input_rid)?;
    if !alarm_on {
        println!("The virtual alarm switch is off; nothing to do.");
        return Ok(());
    }

    let message = "Home alarm triggered by ".to_string()
        + matches.value_of("SENSOR").unwrap();
    message::sendmail(config.email.from.as_str(),
                      config.email.to.as_str(),
                      message,
                      config.smtp)?;
    println!("Email sent to {}.", config.email.to.as_str());

    client.set_switch_value(output_rid, true)?;
    println!("Output switch (rid = {}) turned on.", output_rid);

    Ok(())
}

fn main() {
    let matches = App::new("domo-alarm-sender")
        .version("0.1")
        .author("Christophe Vu-Brugier")
        .about("Notification system for Domoticz to send an email and ring an alarm")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Sets the configuration file")
             .takes_value(true)
             .default_value("/etc/domo-alarm.conf"))
        .arg(Arg::with_name("SENSOR")
             .help("Name of the sensor that triggers the alarm")
             .required(true)
             .index(1))
        .get_matches();

    if let Err(e) = run(matches) {
        println!("Error: {:?}", e);
    }
}
