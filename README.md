# send2kindle

```sh
cargo install send2kindle
```

`send2kindle` is a command-line utility to send documents to your Kindle.

## Prerequisites

- `rustc >= 1.87.0`

## Building

To build the project, navigate to the project directory and run:

```sh
cargo build [--release]
```

## Usage

The program requires several command-line arguments to function:

```sh
send2kindle --smtp-server <SMTP_SERVER_URL> --smtp-username <SMTP_USERNAME> --smtp-password <SMTP_PASSWORD> --kindle-email <KINDLE_EMAIL_ADDRESS> (--file <FILE_PATH> | --stdin) [--filename <FILENAME_OVERRIDE>]
```

Arguments:

- `--smtp-server <SMTP_SERVER_URL>`: The URL of your SMTP server (e.g., `smtp.gmail.com`).
- `--smtp-username <SMTP_USERNAME>`: Your SMTP username.
- `--smtp-password <SMTP_PASSWORD>`: Your SMTP password.
- `--kindle-email <KINDLE_EMAIL_ADDRESS>`: Your Kindle's email address.
- `--file <FILE_PATH>`: Path to the file you want to send.
- `--stdin`: Read the file content from standard input.
- `--filename <FILENAME_OVERRIDE>` (optional): Override the filename for the attachment. If not provided and `--file` is used, the original filename will be used. **This is required if using `--stdin`.**

## Examples

### Send a file

```sh
send2kindle --smtp-server "smtp.gmail.com:587" --smtp-username "your_email@gmail.com" --smtp-password "your_password" --kindle-email "your_kindle@kindle.com" --file ./invoice.test.pdf
```

### Send data from stdin

```sh
cat ./my.pdf | send2kindle --smtp-server "smtp.gmail.com:587" --smtp-username "your_email@gmail.com" --smtp-password "your_password" --kindle-email "your_kindle@kindle.com" --stdin --filename renamed.my.pdf
```
