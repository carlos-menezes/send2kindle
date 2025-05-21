pub struct SmtpConfig<'a> {
    pub server_url: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

pub struct EmailConfig<'a> {
    pub from: &'a str,
    pub to: &'a str,
}

pub struct Config<'a> {
    pub smtp: SmtpConfig<'a>,
    pub email: EmailConfig<'a>,
}

impl<'a> Config<'a> {
    pub fn new(smtp_config: SmtpConfig<'a>, email_config: EmailConfig<'a>) -> Self {
        Config {
            smtp: smtp_config,
            email: email_config,
        }
    }
}
