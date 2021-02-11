use std::env;
use std::fs;

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
    for arg in &args {
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

    for file in files {
        let infile = fs::read_to_string(&file).expect("Something went wrong reading the file");
        let result = infile.split('\n');
        for string in result {
            if is_match(&pattern, &string, &config) {
                let split_string : Vec<&str> = string.split(&pattern).collect();
                if config.H {
                    print!("{}: ", file);
                }
                for i in 0..split_string.len() - 1 {
                    print!("{}\x1b[41m{}\x1b[0m", split_string[i], &pattern);
                }
                println!("{}", split_string[split_string.len() - 1]);
            }
        }
    }
}

fn is_match(pattern : &str, string : &str, config : &Config) -> bool {
    return string.contains(pattern) ^ config.v;
}
