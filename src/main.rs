// main.rs
// vadersay

extern crate failure;
extern crate exitfailure;
extern crate structopt;
extern crate owo_colors;

use failure::ResultExt;
use exitfailure::ExitFailure;
use structopt::StructOpt;
use owo_colors::OwoColorize;


#[derive(StructOpt, Debug)]
struct Settings {

    #[structopt(default_value = "Inspired by danlogs!")]
    message: String,

    #[structopt(long, short)]
    death_star: bool,

    #[structopt(long, short, parse(from_os_str))]
    /// Use your own ASCII art.
    file: Option<std::path::PathBuf>,
}


fn main() -> Result<(), ExitFailure> {

    println!();

    let settings = Settings::from_args();

    if settings.death_star {
        println!("{}", &DEATH_STAR_IMAGE.bright_yellow());
    } else {

        let mut image: String = VADER_IMAGE.to_string();

        match &settings.file {
            Some(path) => {
                image = std::fs::read_to_string(path)
                    .with_context(|_| format!("File not found: {:?}", path))?;
            }

            None => {
                // ignore
            }
        }

        print_message_bubble(&settings.message);
        println!("{}", &image.bright_blue());
    }

    Ok(())
}


fn print_message_bubble(message: &str) {

    let mut actual_message: String = message.to_string();

    if message.len() == 0 {
        actual_message = "...".to_string();
    }

    let length = actual_message.len();

    println!("    {}", "_".repeat(length + 2));
    println!("   /{}\\", " ".repeat(length + 2));
    println!("  |  {}  |", &actual_message);
    println!("   \\{}/", "_".repeat(length + 2));
    println!("    {}|/", " ".repeat((length)/4));
}


// Dart Vader ASCII Art
// https://asciiart.website/index.php?art=movies/star%20wars
const VADER_IMAGE: &'static str = r#" _________________________________ 
|:::::::::::::;;::::::::::::::::::|
|:::::::::::´~||~~~``:::::::::::::|
|::::::::'   .':     o`:::::::::::|
|:::::::' oo | |o  o    ::::::::::|
|::::::: 8  .'.'    8 o  :::::::::|
|::::::: _._| |_,...8    :::::::::|
|::::::'~--.   .--. `.   `::::::::|
|:::::'.    8 =8    ~ \ o ::::::::|
|::::'  ._8  8._ 88.   \ o::::::::|
|:::'       ,.          \ o`::::::|
|:::      . 88`          \  `:::::|
|::'     /. o o \ ::      \88`::::|
|:;     o|| 8 8 |d.        `8 `:::|
|:.       - ^ ^ -'           `-`::|
|:::::.....           ::'     .:::|
|::::::::-'`-        88          `|
|:::::-'.          -       ::     |
|:-~. . .                   :     |
| .. .   ..:   o:8      88o       |
|. .     :::   8:P     d888. . .  |
|_________________________________|
"#;


// Death Star ASCII Art
// https://asciiart.website/index.php?art=movies/star%20wars
const DEATH_STAR_IMAGE: &'static str = r#"           .          .
.          .                  .          .
      +.           _____  .        .
  .        .   ,-~"     "~-.
             ,^ ___         ^. +
            / .^   ^.         \         .
           Y  l  o  !          Y  .
   .       l_ `.___.'        _,[
           |^~"-----------""~ ^|       +
 +       . !                   !     .
        .   \                 /
             ^.             .^
               "-.._____.,-" .
        +           .                .   +
 +          .             +
        .             .      .       -Row
"#;
