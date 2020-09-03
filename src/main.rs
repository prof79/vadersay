// main.rs
// vadersay

extern crate structopt;

use structopt::StructOpt;


#[derive(StructOpt, Debug)]
struct Settings {

    #[structopt(default_value = "Inspired by danlogs!")]
    message: String,

    #[structopt(long, short)]
    death_star: bool,
}


fn main() {

    println!();

    let settings = Settings::from_args();

    if settings.death_star {
        println!("{}", &DEATH_STAR_IMAGE);
    } else {
        print_message_bubble(&settings.message);
        println!("{}", &VADER_IMAGE);
    }
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
