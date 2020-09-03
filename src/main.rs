// main.rs
// vadersay

fn main() {

    const DEFAULT_MESSAGE: &'static str = "Inspired by danlogs!";

    let message: &str = &std::env::args()
        .nth(1)
        .or(Some(DEFAULT_MESSAGE.to_string())).unwrap();

    print_message_bubble(&message);
    println!("{}", &VADER_IMAGE);
}


fn print_message_bubble(message: &str) {

    let mut actual_message: String = message.to_string();

    if message.len() == 0 {
        actual_message = "...".to_string();
    } else if message.len() == 1 {
        actual_message = format!(" {} ", &message);
    }

    let length = actual_message.len();

    println!("");
    println!("    {}", "_".repeat(length));
    println!("   /{}\\", " ".repeat(length));
    println!("  | {} |", &actual_message);
    println!("   \\{}/", "_".repeat(length));
    println!("    {}|/", " ".repeat((length - 2)/4));
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
