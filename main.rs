use std::env;
mod grep;
mod parser;

use parser::parse_input;
use grep::grep;

fn main() {
    let args : Vec<String> = env::args().skip(1).collect();
    let (data, config) = parse_input(args);
    grep(data, config);
}
