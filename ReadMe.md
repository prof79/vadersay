# VADERSAY 0.6.2

Say it like Vader!

This is a Rust CLI program based on a tutorial by [danlogs](https://www.youtube.com/watch?v=De8WG1W2UtM). His was a homage to Pokémon and cowsay, mine is a homage to Star Wars.

## Disclaimer

This program uses ASCII art from "LS" and "Row" that was slightly modified to fit the scenario and achieve better terminal screen output.
The original art can be found in the `ascii` folder of the GitHub repository.

The source of the art is: https://asciiart.website/index.php?art=movies/star%20wars

This software is provided *as-is* under the **MIT License** (see `LICENSE.txt`).

## Command-Line Help

    vadersay 0.6.2
    Say it like Vader!

    USAGE:
        vadersay.exe [FLAGS] [OPTIONS] [message]

    FLAGS:
        -d, --death-star    Vader is visiting the Emperor aboard the Death Star and can't talk right now
        -h, --help          Prints help information
        -s, --stdin         Read the message from standard input (pipe) instead
        -V, --version       Prints version information

    OPTIONS:
        -f, --file <file>    Use your own ASCII art from a text file

    ARGS:
        <message>    Message to print [default: Inspired by danlogs!]

      ______________________________________
     /                                      \
    |  Impressive ... *most* impressive ...  |
     \______________________________________/
            |/
    _________________________________
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
    |:::::.....           ::'    ..:::|
    |::::::::-'`-        88          `|
    |:::::-'.          -       ::     |
    |:-~. . .                   :     |
    | .. .   ..:   o:8      88o       |
    |. .     :::   8:P     d888. . .  |
    |.      .8P   d8P    based on LS  |
    |_________________________________|
