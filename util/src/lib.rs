
use std::{
    fs::File,
    io::{prelude::*, BufReader, Cursor},
    path::Path,
};
use regex::Regex;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use walkdir::{DirEntry, WalkDir};
use tokio::fs::{metadata, self};
use zip_extract::extract;
use std::path::PathBuf;


pub fn fetch_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub async fn write_file(filename: impl AsRef<Path>, text: &str) {
    let file_writer = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .await;
    if let Ok(mut fs) = file_writer {
        fs.write_all(text.as_bytes()).await.unwrap();
    }
}
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)

}

pub async fn search_extract(src: Vec<u8>, count: u32) -> Vec<String> {
    let token_regex = Regex::new(r#"[A-z|0-9]{24,26}\.[A-z|0-9|\W]{6}\.[A-z|0-9|\W]{27,38}"#).unwrap();
    extract(Cursor::new(src),  &PathBuf::from("./data"), false).unwrap();
    let walker = WalkDir::new("./data").into_iter();
    let mut tokens = Vec::new();

    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let path = match &entry.as_ref() {
            Ok(x) => x.path(),
            Err(_) => continue,
        };
        let md = metadata(&path).await;
        if let Ok(md) = md {
            if md.is_file() {
                if let Ok(file) = fs::read_to_string(&path).await {
                    for token in token_regex.find_iter(&file) {
                        tokens.push(token.as_str().to_string());
                    }
                };
            }
        }
    }
    if let Err(e) = fs::remove_dir_all("./data").await {
        println!("\x1b[0;91mError: {e}\x1b[0m");
    };
    if let Err(e) = fs::create_dir("./data").await {
        println!("\x1b[0;91mError: {e}\x1b[0m");
    }
    println!("\x1b[0;92mFinished searching extract {count}...\x1b[0m");
    tokens
}