extern crate ansi_term;
use std::io;
use std::io::prelude::*;
use ansi_term::Colour::*;

fn mlt(symbol: &str, length: i32) -> String {
    let mut string = String::from("");
    while string.len() < length as usize {
        string.push(symbol.parse().unwrap());
    }
    return string;
}

fn print_center(message: String, length: i32, size: i32) {
    if length < size {
        let point = (size - length) / 2;
        println!("{}{}", mlt(" ", point), message);
    } else {
        println!("{}", message);
    }
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press enter to exit...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    println!();
    let spaces = 15;
    let mut leaves = String::from("0");
    println!("{}{}", mlt(" ", spaces), Yellow.bold().paint("X"));
    for index in 0..spaces {
        println!("{}{}", mlt(" ", spaces - index), Green.paint(leaves.as_str()));
        leaves.push_str("10");
    }
    for _ in 0..(spaces / 2) {
        println!("{}{}", mlt(" ", spaces - 1), RGB(153, 102, 51).paint("101"));
    }
    println!();
    let mut message = "!Merry Christmas¡";
    print_center(Yellow.paint(message).to_string(), message.len() as i32, leaves.len() as i32);
    message = "And a happy new year";
    print_center(Green.paint(message).to_string(), message.len() as i32, leaves.len() as i32);
    pause();
}
