# send2kindle

Still a work in progress.

[Check your Kindle email and whitelisted emails here.](https://www.amazon.com/hz/mycd/myx#/home/settings/payment)

```sh
cargo run -- --smtp-username "your-email@gmail" --smtp-password "your-app.password" --to-email "your_amazon_email+amazon_id@kindle.com" --from-email "kindle_whitelisted_email" --smtp-server "smtp_server_ip" --stdin
```
