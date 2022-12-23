use colorful::{Colorful, Color};
use input_macro::input;
pub fn clear() {
  if cfg!(target_os = "windows") {
    std::process::Command::new("cls").status().unwrap();
  } else {
    std::process::Command::new("clear").status().unwrap();
  }
}

pub fn cli() {
  println!("{}\n",
  "              ▀██▀▀█▄    ▄▄█▀▀██   ▀██▀     █▀▀██▀▀█\n               ██   ██  ▄█▀    ██▄  ██         ██\n               ██▀▀▀█▄  ██      ██  ██         ██\n               ██    ██ ▀█▄     ██  ██         ██\n              ▄██▄▄▄█▀   ▀▀█▄▄▄█▀  ▄██▄▄▄▄▄█  ▄██▄".gradient(Color::LightCyan)
  );
}


pub fn options() {
  println!("{}\n\n",
"╔════════════════════════════════╬═════════════════════════════════╗
║  1. Scrape forks URL                 3. Auto scrape              ║
║  2. Check tokens in tokens.txt       4. Scrape User repls        ║
╚════════════════════════════════╬═════════════════════════════════╝".gradient(Color::LightCyan))
}


pub fn input(prompt: &str, example: Option<&str>) -> String {
  let gradient = Color::LightCyan1;
  if let Some(eg) = example {

    let message = format!(
"╔═══ ╬ {prompt} ╬ {eg} ╬
║
╚══[>]");

    let val = input!("{} ", message.gradient(gradient));
    println!();

    val
  } else {

    let message = format!(
"╔═══ ╬ {prompt} ╬
║
╚══[>]");

    let val = input!("{} ", message.gradient(gradient));
    println!();

    val
  }

}