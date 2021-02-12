use std::env;
use std::fs;
use std::io::{self, BufRead};

struct Config {
    v : bool,
    h : bool,
    H : bool,
}

fn main() {
    let mut config = Config {
        v : false,
        h : true,
        H : false,
    };

    let args : Vec<String> = env::args().skip(1).collect();
    let mut options : Vec<String> = Vec::new();
    let mut files : Vec<String> = Vec::new();
    let mut pattern : String = "".to_string();
    for arg in args {
        if pattern == "" && arg.chars().next().unwrap().to_string() != "-" {
            pattern = arg.to_string();
            continue;
        }

        if arg.chars().next().unwrap().to_string() == "-" {
            options.push(arg.to_string());
            continue;
        }

        if arg.chars().next().unwrap().to_string() != "-" {
            files.push(arg.to_string());
            continue;
        }
    }

    if files.len() > 1 {
        config.H = true;
        config.h = false;
    }

    for option in options {
        match &*option {
            "-v" => {
                config.v = true;
            },

            "-h" => {
                config.h = true;
                config.H = false;
            },

            "-H" => {
                config.H = true;
                config.h = false;
            },

            _ => {
                panic!("unknown option {}", option);
            }
        }
    }

    if files.len() == 0 {
        let stdin = io::stdin();
        for string in stdin.lock().lines() {
            match string {
                Ok(string) => {
                    if is_match(&pattern, &string, &config) {
                        println!("{}", get_result_string(&pattern, &string, "stdin", &config));
                    }
                },
                Err(_) => panic!("error with read line from stdin"),
            }
        }
    }

    for file in files {
        let infile = fs::read_to_string(&file).expect("error with read file");
        let result = infile.split('\n');
        for string in result {
            if is_match(&pattern, &string, &config) {
                println!("{}", get_result_string(&pattern, &string, &file, &config));
            }
        }
    }
}

fn is_match(pattern : &str, string : &str, config : &Config) -> bool {
    return string.contains(pattern) ^ config.v;
}

fn get_result_string(pattern : &str, string : &str, filename : &str, config : &Config) -> String {
    let mut result_string : String = "".to_owned();
    let split_string : Vec<&str> = string.split(&pattern).collect();

    if config.H {
        result_string = format!("{}: ", &filename);
    }

    for i in 0..split_string.len() - 1 {
        result_string = format!("{}{}\x1b[41m{}\x1b[0m", result_string, split_string[i], &pattern);
    }

    result_string = format!("{}{}", result_string, split_string[split_string.len() - 1]);
    return result_string;
}
