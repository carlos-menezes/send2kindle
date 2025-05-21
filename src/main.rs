mod cli;
mod config;
mod email;
mod files;

use clap::Parser;
use std::process::exit;

fn main() {
    let args = cli::CliArgs::parse();

    let config = config::Config::new(
        config::SmtpConfig {
            server_url: &args.smtp_server,
            username: &args.smtp_username,
            password: &args.smtp_password,
        },
        config::EmailConfig {
            from: &args.from_email,
            to: &args.to_email,
        },
    );

    let filename = match &args.filename {
        Some(filename_override) => filename_override,
        None => {
            if args.stdin {
                // If `stdin` is used, create a file name with the date and time
                // in the format YYYY-MM-DD_HH-MM-SS.pdf
                let now = chrono::Local::now();
                let out = now.format("%Y-%m-%d_%H-%M-%S");
                &format!("{}.pdf", out)
            } else if args.file.is_some() {
                // if a file is provided, use the filename
                &args.file.clone().unwrap()
            } else {
                println!("[error] --stdin or --file must be passed");
                exit(1)
            }
        }
    };

    let mut filebody: Vec<u8> = Vec::new();

    match args.stdin {
        true => files::read_from_stdin(&mut filebody).expect("Failed to read from stdin"),
        false => (),
    }

    match args.file {
        Some(file) => files::read_from_file(&file, &mut filebody).unwrap_or_else(|error| {
            println!("[error] {}", error);
            exit(1);
        }),
        None => (),
    }

    if !files::is_pdf_file(&filebody) {
        println!("[error] `{}` is not a PDF", filename);
    }

    println!("[info] sending {} to Kindle", filename);

    let email = email::build_email(&filename, &filebody, &config).unwrap_or_else(|error| {
        println!("[error] {}", error);
        exit(1);
    });

    email::send_email(&email, &config).unwrap_or_else(|error| {
        println!("[error] {}", error);
        exit(1);
    });

    println!("[info] sent2kindle!");
}
