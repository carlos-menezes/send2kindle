use clap::{Parser, command};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct CliArgs {
    #[arg(long, short, required = false, required_unless_present("stdin"))]
    pub(crate) file: Option<String>,

    #[arg(long, short)]
    pub(crate) stdin: bool,

    #[arg(long, required_if_eq("stdin", "true"))]
    pub(crate) filename: Option<String>,

    #[arg(long)]
    pub(crate) smtp_username: String,

    #[arg(long)]
    pub(crate) smtp_password: String,

    #[arg(long)]
    pub(crate) smtp_server: String,

    #[arg(long)]
    pub(crate) kindle_email: String,
}
