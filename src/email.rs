use std::process::exit;

use lettre::{
    Message, SmtpTransport, Transport,
    error::Error,
    message::{
        Attachment, Mailbox, MultiPart, SinglePart,
        header::{self, ContentType},
    },
    transport::smtp::authentication::Credentials,
};

use crate::config;

pub(crate) fn build_email(
    filename: &str,
    filebody: &Vec<u8>,
    config: &config::Config,
) -> Result<Message, Error> {
    let content_type = ContentType::parse("application/octet-stream").unwrap();
    let attachment = Attachment::new(filename.to_owned()).body(filebody.clone(), content_type);

    let from = Mailbox::new(
        None,
        config.smtp_config.username.parse().unwrap_or_else(|error| {
            println!("[error] {}", error);
            exit(1);
        }),
    );

    let to = Mailbox::new(
        None,
        config.kindle_email.parse().unwrap_or_else(|error| {
            println!("[error] {}", error);
            exit(1);
        }),
    );

    Message::builder()
        .from(from)
        .to(to)
        .subject(format!("{} to Kindle from send2kindle", filename))
        .multipart(
            MultiPart::mixed()
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(String::from(format!(
                            "Sending {} to your Kindle.",
                            filename
                        ))),
                )
                .singlepart(attachment),
        )
}

pub(crate) fn send_email(
    email: &Message,
    config: &config::Config,
) -> Result<lettre::transport::smtp::response::Response, lettre::transport::smtp::Error> {
    let credentials = Credentials::new(
        config.smtp_config.username.to_owned(),
        config.smtp_config.password.to_owned(),
    );

    let mailer = SmtpTransport::relay(&config.smtp_config.server_url)?
        .credentials(credentials)
        .build();

    mailer.send(email)
}
