# send2kindle

Send any [supported file](https://www.amazon.com/gp/help/customer/display.html?nodeId=G5WYD9SAF7PGXRNA) to Kindle via SMTP.

```sh
send2kindle --smtp-username "my-gmail@gmail.com" --smtp-password "my-app-password" --smtp-server "smtp.gmail.com" --kindle-email "my-amazon-email+amazon_id@kindle.com" <--stdin | --file> [--filename "send2kindle.pdf"]
```

## Running

### Prerequisites

- `--smtp-username` must be whitelisted in the Kindle email settings:
  - Check your Kindle email[0] and whitelisted emails [here](https://www.amazon.com/hz/mycd/myx#/home/settings/payment);
    - [0]: Optionally, check your Kindle settings.
- If using Google Mail, you may need to pass an [App Password](https://support.google.com/accounts/answer/185833?hl=en) instead of your password in `--smtp-password`.

### Read file from filename

TODO.

### Input from `stdin`

When using `--stdin`, `--filename` must be specified **including an extension**.

TODO.
