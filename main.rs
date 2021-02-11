use std::env;
use std::fs;

struct Config {
    v : bool,
}

fn main() {
    let mut config = Config {
        v : false,
    };

    let args : Vec<String> = env::args().skip(1).collect();
    let mut params : Vec<String> = Vec::new();
    let mut files : Vec<String> = Vec::new();
    let mut substring : String = "".to_string();
    for arg in &args {
        if substring == "" && arg.chars().next().unwrap().to_string() != "-" {
            substring = arg.to_string();
            continue;
        }

        if arg.chars().next().unwrap().to_string() == "-" {
            params.push(arg.to_string());
            continue;
        }

        if arg.chars().next().unwrap().to_string() != "-" {
            files.push(arg.to_string());
            continue;
        }
    }

    for param in params {
        if param == "-v" {
            config.v = true;
        }
    }

    for file in files {
        let infile = fs::read_to_string(&file).expect("Something went wrong reading the file");
        let result = infile.split('\n');
        for string in result {
            if is_match(&substring, &string, &config) {
                let b : Vec<&str> = string.split(&substring).collect();
                    for i in 0..b.len() - 1 {
                        print!("{}\x1b[41m{}\x1b[0m", b[i], &substring);
                    }
                println!("{}", b[b.len() - 1]);
            }
        }
    }
}

fn is_match(substring : &str, string : &str, config : &Config) -> bool {
    return string.contains(substring) ^ config.v;
}
