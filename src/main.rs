use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn lint_file(name: &str) {
    let f = File::open(name).expect("Unable to open file");
    let f = BufReader::new(f);
    let mut config_part: bool = false;
    let mut last_line_comma: bool = false;
    for (index, line) in f.lines().enumerate() {
        let line = line.expect("Unable to read line");
        let line_contents = line;
        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();
        let last_char: Option<char> = line_contents.chars().next_back();
        match first_char.pop() {
            Some('{') => {
                config_part = true;
                continue;
            }
            Some(' ') => {
                if config_part && last_char.unwrap() == ',' {
                    last_line_comma = true;
                    continue;
                }
                last_line_comma = false;
            }
            Some('}') => {
                config_part = false;
                if last_line_comma {
                    println!("Error in line {}, extra comma", index);
                    last_line_comma = false;
                    continue;
                }
            }
            _ => continue,
        }
    }
}

fn usage() {
    println!("Rust linter, a naive marathon template linter");
    println!("Usage: rust-linter <somefile.tpl>")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => lint_file(&args[1]),
        _ => usage(),
    }
}
