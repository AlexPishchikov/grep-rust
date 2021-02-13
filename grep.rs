use std::fs;
use std::io::{self, BufRead};

use parser::{Data, Config};

pub fn grep(data : Data, config : Config) {
    if data.files.len() == 0 {
        stdin_grep(data, config);
    }
    else {
        file_grep(data, config);
    }
}

fn stdin_grep(data : Data, config : Config) {
    let mut num_matches = 0;
    let stdin = io::stdin();
    for string in stdin.lock().lines() {
        match string {
            Ok(string) => {
                if is_match(&data.pattern, &string, &config) {
                    num_matches += 1;
                    if !config.c {
                        println!("{}", get_result_string(&data.pattern, &string, "stdin", &config));
                    }
                    if num_matches >= config.num {
                        break;
                    }
                }
            },
            Err(_) => panic!("error with read line from stdin"),
        }
    }

    if config.c {
        println!("{}", get_result_string_c(num_matches, "stdin", &config));
    }
}

fn file_grep(data : Data, config : Config) {
    let mut num_matches_m = 0;
    for file in data.files {
        let mut num_matches_c = 0;
        let infile = fs::read_to_string(&file).expect("error with read file");
        let result = infile.split('\n');
        for string in result {
            if is_match(&data.pattern, &string, &config) {
                if config.c {
                    num_matches_m += 1;
                    num_matches_c += 1;
                }
                else {
                    println!("{}", get_result_string(&data.pattern, &string, &file, &config));
                    num_matches_m += 1;
                }
                if num_matches_m >= config.num {
                    break;
                }
            }
        }
        if config.c {
            println!("{}", get_result_string_c(num_matches_c, &file, &config));
        }
        if num_matches_m >= config.num {
            break;
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
