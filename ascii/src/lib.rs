use colorful::{Colorful, Color};

pub fn clear() {
  if cfg!(target_os = "windows") {
    std::process::Command::new("cls").status().unwrap();
  } else {
    std::process::Command::new("clear").status().unwrap();
  }
}

pub fn cli() {
    println!("{}", "
     ▀██▀▀█▄    ▄▄█▀▀██   ▀██▀     █▀▀██▀▀█
      ██   ██  ▄█▀    ██   ██         ██
      ██▀▀▀█▄  ██      ██  ██         ██
      ██    ██ ▀█▄     ██  ██         ██
     ▄██▄▄▄█▀   ▀▀█▄▄▄█▀  ▄██▄▄▄▄▄█  ▄██▄
    ".gradient_with_color(Color::LightCyan, Color::Magenta));
}


pub fn options() {
    println!("{}", "
    ╔══════════════════╬═════════════════╗
    ║  1. Scrape forks from inputted URL ║
    ╬  2. Check the tokens in tokens.txt ╬
    ║  3. Auto scrape from query         ║
    ╚══════════════════╬═════════════════╝
    "
    .gradient_with_color(Color::LightCyan, Color::LightMagenta));
}
