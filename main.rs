use std::env;
use std::fs;
use std::io::{self, BufRead};

struct Config {
    v : bool,
    h : bool,
    H : bool,
    c : bool,
    m : bool,
    num : i32,
}

fn main() {
    let mut config = Config {
        v : false,
        h : true,
        H : false,
        c : false,
        m : false,
        num : i32::MAX,
    };

    let args : Vec<String> = env::args().skip(1).collect();

    let mut options : Vec<String> = Vec::new();
    let mut files : Vec<String> = Vec::new();
    let mut pattern : String = "".to_string();

    let mut i = 0;
    while i < args.len() {
        if pattern == "" && args[i].chars().next().unwrap().to_string() != "-" {
            pattern = args[i].to_string();
            i += 1;
            continue;
        }

        if args[i].chars().next().unwrap().to_string() == "-" {
            options.push(args[i].to_string());
            if options.last().unwrap() == &"-m".to_string() {
                i += 1;
                let num = args[i].parse::<i32>();
                match num {
                    Ok(num) => config.num = num,
                    Err(_)  => panic!("error with option -m"),
                }
            }
            i += 1;
            continue;
        }

        if args[i].chars().next().unwrap().to_string() != "-" {
            files.push(args[i].to_string());
            i += 1;
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

            "-c" => {
                config.c = true;
            },

            "-m" => {
                config.m = true;
            },

            _ => {
                panic!("unknown option {}", option);
            }
        }
    }

    let mut num_matches_c = 0;

    if files.len() == 0 {
        let mut num_matches_m = 0;
        let stdin = io::stdin();
        for string in stdin.lock().lines() {
            if num_matches_c >= config.num {
                break;
            }
            match string {
                Ok(string) => {
                    if is_match(&pattern, &string, &config) {
                        if config.c {
                            num_matches_c += 1;
                            num_matches_m += 1;
                        }
                        else {
                            num_matches_c += 1;
                            println!("{}", get_result_string(&pattern, &string, "stdin", &config));
                        }
                    }
                    if num_matches_c >= config.num {
                        break;
                    }
                },
                Err(_) => panic!("error with read line from stdin"),
            }
        }

        if config.c {
            println!("{}", get_result_string_c(num_matches_m, "stdin", &config));
        }
    }

    for file in files {
        let mut num_matches_m = 0;
        if num_matches_c >= config.num {
            break;
        }
        let infile = fs::read_to_string(&file).expect("error with read file");
        let result = infile.split('\n');
        for string in result {
            if is_match(&pattern, &string, &config) {
                if config.c {
                    num_matches_c += 1;
                    num_matches_m += 1;
                }
                else {
                    println!("{}", get_result_string(&pattern, &string, &file, &config));
                    num_matches_c += 1;
                    if num_matches_c >= config.num {
                        break;
                    }
                }
                if num_matches_c >= config.num {
                    break;
                }
            }
        }
        if config.c {
            println!("{}", get_result_string_c(num_matches_m, &file, &config));
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

fn get_result_string_c(num : i32, filename : &str, config : &Config) -> String {
    let mut result_string : String = "".to_owned();
    if config.H {
        result_string = format!("{}: ", &filename);
    }
    result_string = format!("{}{}", result_string, num);
    return result_string;
}
