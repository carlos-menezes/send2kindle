pub struct SmtpConfig<'a> {
    /// The SMTP server to use for sending the email.
    /// example: smtp.gmail.com
    pub server_url: &'a str,
    /// The email address to send the file from.
    /// This should be the same email address as the one used to log in to the SMTP server.
    /// This email must be whitelisted in the [Kindle settings](https://www.amazon.com/hz/mycd/myx#/home/settings/payment).
    pub username: &'a str,
    /// The password for the associated `username` in the SMTP server.
    /// If using Gmail, you may need to create an [App Password](https://support.google.com/accounts/answer/185833?hl=en).
    pub password: &'a str,
}

pub struct Config<'a> {
    /// The SMTP configuration for sending the email.
    pub smtp_config: SmtpConfig<'a>,
    /// The email address of the Kindle device.
    pub kindle_email: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(smtp_config: SmtpConfig<'a>, kindle_email: &'a str) -> Self {
        Config {
            smtp_config,
            kindle_email,
        }
    }
}
