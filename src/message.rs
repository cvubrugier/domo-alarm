extern crate lettre;

use config::SmtpConfig;
use error::DomoError;
use lettre::email::EmailBuilder;
use lettre::transport::EmailTransport;
use lettre::transport::smtp::{SecurityLevel, SmtpTransportBuilder};
use lettre::transport::smtp::authentication::Mechanism;

pub fn sendmail<S1, S2, S3>(from: S1, to: S2, message: S3, smtp_config: SmtpConfig) -> Result<(), DomoError>
    where S1: Into<String>,
          S2: Into<String>,
          S3: Into<String> {
    let email = EmailBuilder::new().to((to.into(), ""))
        .from(from.into())
        .subject(message.into())
        .build()?;

    let mut mailer = SmtpTransportBuilder::new((smtp_config.host.as_str(), smtp_config.port))
        .unwrap()
        .credentials(smtp_config.user.as_str(), smtp_config.password.as_str())
        .security_level(SecurityLevel::Opportunistic)
        .authentication_mechanism(Mechanism::Plain)
        .build();

    mailer.send(email)?;

    return Ok(());
}
