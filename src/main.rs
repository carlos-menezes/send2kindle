mod cli;
mod config;
mod email;
mod files;

use clap::Parser;
use std::process::exit;

fn determine_filename(args: &cli::CliArgs) -> &String {
    match &args.filename {
        Some(filename_override) => filename_override,
        None => {
            if args.file.is_some() {
                // if a file is provided, use the filename
                args.file.as_ref().unwrap()
            } else {
                println!(
                    "[error] --stdin or --file must be passed when --filename is not provided"
                );
                exit(1)
            }
        }
    }
}

fn read_file_content(args: &cli::CliArgs, filebody: &mut Vec<u8>) {
    match args.stdin {
        true => files::read_from_stdin(filebody).expect("Failed to read from stdin"),
        false => {
            // Unwrapping the value of `args.file` is safe here.
            // If `stdin` does not exist, `file` must, otherwise clap's parsing will fail.
            files::read_from_file(args.file.as_ref().unwrap(), filebody).unwrap_or_else(|error| {
                println!("[error] {}", error);
                exit(1);
            })
        }
    }
}

fn main() {
    let args = cli::CliArgs::parse();

    let config = config::Config::new(
        config::SmtpConfig {
            server_url: &args.smtp_server,
            username: &args.smtp_username,
            password: &args.smtp_password,
        },
        &args.kindle_email,
    );

    let filename = determine_filename(&args);
    let mut filebody: Vec<u8> = Vec::new();
    read_file_content(&args, &mut filebody);

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
