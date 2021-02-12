pub struct Config {
    pub v : bool,
    pub h : bool,
    pub H : bool,
    pub c : bool,
    pub m : bool,
    pub num : i32,
}

pub struct Data {
    pub options : Vec<String>,
    pub files : Vec<String>,
    pub pattern : String,
}

pub fn parse_input(args : Vec<String>) -> (Data, Config) {
    let mut config = Config {
        v : false,
        h : true,
        H : false,
        c : false,
        m : false,
        num : i32::MAX,
    };

    let mut data = Data {
        options : Vec::new(),
        files : Vec::new(),
        pattern : "".to_string(),
    };

    let mut i = 0;
    while i < args.len() {
        let arg_char = args[i].chars().next();
        let mut arg : String = "".to_string(); 
        match arg_char {
            Some(arg_char) => arg = arg_char.to_string(),
            None => panic!("error with parse input"),
        }

        if data.pattern == "" && arg != "-" {
            data.pattern = args[i].to_string();
            i += 1;
            continue;
        }

        if arg == "-" {
            data.options.push(args[i].to_string());
            if data.options.last().unwrap() == &"-m".to_string() {
                i += 1;
                let num = args[i].parse::<i32>();
                match num {
                    Ok(num)  => config.num = num,
                    Err(num) => panic!("error with option -m: {:?}", num),
                }
            }
            i += 1;
            continue;
        }

        if arg != "-" {
            data.files.push(args[i].to_string());
            i += 1;
            continue;
        }
    }

    if data.files.len() > 1 {
        config.H = true;
        config.h = false;
    }

    for option in data.options.clone() {
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

    return (data, config);
}
