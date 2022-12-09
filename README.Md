# Bolt Scraper
The Fastest Replit.com discord token scraper. Designed to gather discord tokens from replit, using multiple different methods.

## Setup

If you want to, you may add your desired webhook url to config.json.

Requires Rust installed with Cargo. You can follow this [guide](https://doc.rust-lang.org/cargo/getting-started/installation.html) here. Then please set rust to the nightly branch using `rustup default nightly`. After that you may run `cargo run --release` and should start running.

Once your script is running, you paste the repl in which you want to scrape the forks of. For example /@templates/Discord-Bot-Starter. Hit enter, and it will begin. This will also check the tokens for you and place them in `valid.txt`.

You may need to increase your File Descriptor limit for your PC if you are doing more heavy duty scraping ~ 1500+. If not you may run into errors when attempting to extract.

**Disclaimer: This tool generally requires a decent internet connection.**

## Features

- [x] Blazing fast speeds 🔥
- [x] Inbuilt Token checker - with webhook sending
- [x] Auto-scraping 🤖
- [ ] Proxy Support (upcoming)
- [ ] Token Information (upcoming)


## Help & Support
- My [discord server](https://discord.gg/jD4C57AJg6)