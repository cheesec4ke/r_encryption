use const_hex::decode;
use rand::prelude::*;
use std::{env, error::Error, fs, fs::File, io, io::Write, process, str};
use std::env::Args;
use std::iter::Skip;

fn main() {
    let args = env::args().skip(1);
    let config = Config::build(args);

    if config.interactive {
        interactive();
    } else {
        if let Err(e) = quiet(config) {
            println!("Error: {e}");
            process::exit(0);
        }
    }
}

struct Config {
    in_path: String,
    op: String,
    key: u8,
    out_path: String,
    interactive: bool
}

impl Config {
    fn build(mut args: Skip<Args>) -> Config {
        let mut in_path = String::from("_");
        let mut op = String::from("E");
        let mut key: u8 = 1;
        let mut out_path = String::from("_");
        let mut interactive= true;

        'parse: while let Some(arg) = args.next() {
            match &arg[..] {
                "-h" | "--help" => help(),
                "-i" | "--interactive" => {
                    break 'parse;
                }
                "-q" | "--quiet" => {
                    interactive = false;
                }
                "-e" | "--encrypt" => {
                    op = "E".to_string();
                    if let Some(arg_input) = args.next() {
                        in_path = arg_input;
                    } else {
                        println!("No input path specified");
                        process::exit(0)
                    }
                }
                "-d" | "--decrypt" => {
                    op = "D".to_string();
                    if let Some(arg_input) = args.next() {
                        in_path = arg_input;
                    } else {
                        println!("No input path specified");
                        process::exit(0)
                    }
                }
                "-k" | "--key" => {
                    if let Some(arg_key) = args.next() {
                        match arg_key.parse::<u8>() {
                            Ok(k) => {
                                key = k;
                            }
                            _ => {
                                println!("Invalid key \"{arg_key}\", using default \"1\"\n");
                            }
                        }
                    } else {
                        println!("No value specified for parameter --key, using default \"3\"\n");
                    }
                }
                "-o" | "--output" => {
                    if let Some(arg_config) = args.next() {
                    out_path = arg_config;
                    } else {
                        println!("No value specified for parameter --output_path");
                    }
                }
                _ => {
                    if arg.starts_with('-') {
                        println!("Unknown argument {}", arg);
                    } else {
                        println!("Unknown positional argument {}", arg);
                    }
                }
            }
        }

        Config {in_path, op, key, out_path, interactive}
    }
}

fn help() {
    println!("Usage: encryption [OPTIONS]\n");
    println!("Options:");
    println!("  -h, --help                             Displays this list of options");
    println!("  -i, --interactive   (Default)          Runs in interactive mode, no further options are used");
    println!("  -q, --quiet                            Runs in quiet mode with the options below:\n");
    println!("  -e, --encrypt <INPUT_PATH>             Encrypts file at <INPUT_PATH>");
    println!("  -d, --decrypt <INPUT_PATH>             Decrypts the text file at <INPUT_PATH>");
    println!("  -k, --key <KEY, 0..255>    (Optional)  Uses <KEY> as encryption length [default: 3]");
    println!("  -o, --output <OUTPUT_PATH> (Optional)  Writes the output to a file at <PATH> instead of the console (will overwrite existing files)");
    process::exit(0);
}

fn quiet(config: Config) -> Result<String, Box<dyn Error>> {
    if config.op == "E" {
        match enc("_", &config.in_path, &config.out_path, config.key) {
            Ok(encrypted) => {
                if config.out_path == ("_") {
                    println!("Encrypted text: {encrypted}");
                } else {
                    println!("Successfully wrote encrypted text to {}", config.out_path);
                }
                process::exit(0);
            }
            Err(error) => {
                println!("Encryption error: {error}");
                process::exit(0);
            }
        }
    } else if config.op == "D" {
        match dec("_", &config.in_path, &config.out_path) {
            Ok(result) => {
                if config.out_path.starts_with("_") {
                    println!("Decrypted text: {result}");
                } else {
                    println!("Successfully wrote decrypted text to {}", config.out_path);
                }
                process::exit(0);
            }
            Err(error) => {
                println!("Input file error: {error}");
                process::exit(0);
            }
        }
    } else {
        println!("No valid operation selected");
        process::exit(0);
    }
}

fn interactive() {
    'outer: loop {
        print!("Input 1 for encryption, 2 for decryption, or 3 to exit: ");
        io::stdout().flush().unwrap();

        'inner: loop {
            let mut input = String::new();
            let _ = io::stdin().read_line(&mut input);

            match input.trim().parse::<u8>() {
                Ok(1) => {
                    print!("Input text to encrypt: ");
                    io::stdout().flush().unwrap();

                    let mut text = String::new();
                    let _ = io::stdin().read_line(&mut text);

                    print!("Input encryption length from 0 to 255: ");
                    io::stdout().flush().unwrap();


                    'key_loop: loop {
                        let mut key = String::new();
                        let _ = io::stdin().read_line(&mut key);

                        if key.trim() == "C" || key.trim() == "c"{  }

                        match key.trim().parse::<u8>() {
                            Ok(k) => {
                                match enc(&text.trim(), "_", "_", k) {
                                    Ok(result) => {
                                        println!("Encrypted text: {result}");
                                    }
                                    Err(error) => {
                                        println!("Error: {error}")
                                    }
                                }

                                break 'key_loop;
                            }
                            _ => {
                                print!("Please input a number from 0 to 255 or 'C' to cancel: ");
                                io::stdout().flush().unwrap();
                            }
                        }
                    }

                    break 'inner;
                }
                Ok(2) => {
                    print!("Input string to decrypt: ");
                    io::stdout().flush().unwrap();

                    'dec_loop: loop {
                        let mut str = String::new();
                        let _ = io::stdin().read_line(&mut str);
                        if str.trim() == "C" || str.trim() == "c" {
                            break 'dec_loop;
                        }

                        match dec(&str.trim(), "_", "_") {
                            Ok(result) => {
                                println!("Decrypted text: {result}");

                                break 'dec_loop;
                            }
                            Err(error) => {
                                print!("Error: {error}\nInput C to cancel or input a valid string to continue: ");
                                io::stdout().flush().unwrap();
                            }
                        }
                    }

                    break 'inner;
                }
                Ok(3) => {
                    println!("o/");

                    break 'outer;
                }
                _ => {
                    print!("Please input 1, 2, or 3: ");
                    io::stdout().flush().unwrap();
                }
            }
        }
    }
}

fn enc(input: &str, in_path: &str, out_path: &str, key: u8) -> Result<String, Box<dyn Error>> {
    //debug: println!("\ni:{}\nip:{}\nop:{}\nk:{}\n", input, in_path, out_path, key);

    let mut out= String::new();

    let hex_key = format!("{form:0>2}", form = format!("{key:x}"));
    out.push(hex_key.chars().next().unwrap());

    let mut rng = thread_rng();

    if in_path != ("_") {
        let file_input = fs::read(in_path)?;

        if ! out_path.starts_with('_') {
            let mut out_file = File::create(out_path)?;
            out_file.write_all(out.as_bytes())?;

            for n in 0..file_input.len() {
                let hex_num = format!("{:x}", &file_input[n]);
                let hex_num_v = format!("{:0>4}", hex_num).into_bytes();

                for n in 0..4 {
                    out_file.write_all(&[hex_num_v[n]])?;

                    for _n in 0..key {
                        out_file.write_all((&format!("{:x}", rng.gen_range(0..=15))).as_bytes())?;
                    }
                }
            }

            out_file.write_all(&[hex_key.as_bytes()[1]])?;
        } else {
            for n in 0..file_input.len() {
                let hex_num = format!("{:x}", file_input[n]);
                let hex_num_v = format!("{:0>4}", hex_num).into_bytes();

                for n in 0..4 {
                    out.push_str(&String::from_utf8_lossy(&[hex_num_v[n]]));

                    for _n in 0..key {
                        out.push_str(&format!("{:x}", rng.gen_range(0..=15)));
                    }
                }
            }
            out.push(hex_key.chars().nth(1).unwrap());
        }
    } else if input != ("_"){
        let input = input.to_owned().into_bytes();

        for n in 0..input.len() {
            let hex_num = format!("{:x}", input[n]);
            let hex_num_v = format!("{:0>4}", hex_num).into_bytes();

            for n in 0..4 {
                out.push_str(&String::from_utf8_lossy(&[hex_num_v[n]]));

                for _n in 0..key {
                    out.push_str(&format!("{:x}", rng.gen_range(0..=15)));
                }
            }
        }
        out.push(hex_key.chars().nth(1).unwrap());
    }

    Ok(out)
}

fn dec(input: &str, in_path: &str, out_path: &str) -> Result<String, Box<dyn Error>> {
    //debug: println!("\ni:{}\nip:{}\nop{}\n", input, in_path, out_path);

    let mut out = String::new();
    let mut pos: i32 = 1;

    if in_path != "_" {
        let file_input = fs::read(in_path)?;

        let key = i32::from_str_radix(&(String::from_utf8_lossy(&[file_input[0]])
            + &*String::from_utf8_lossy(&[file_input[file_input.len() - 1]])), 16)? + 1;

        let real_chars = (file_input.len() - 2) as i32 / (key * 4) - 1;

        if out_path != "_" {
            let mut file = File::create(out_path)?;
            for _n in 0..real_chars {
                let mut hex_char = String::new();
                for _ in 0..4 {
                    hex_char.push_str(&String::from_utf8_lossy(&[file_input[pos as usize]]));
                    pos += key;
                }
                file.write_all((&decode(&hex_char)?.drain(1..)).as_ref())?;
            }
        } else {
            for _n in 0..real_chars {
                let mut hex_char = String::new();
                for _ in 0..4 {
                    hex_char.push_str(&String::from_utf8_lossy(&[file_input[pos as usize]]));
                    pos += key;
                }
                out.push_str(&String::from_utf8(decode(&hex_char)?)?);
            }
        }
    } else if input != "_" {
        let input = input.to_owned().into_bytes();

        let key = i32::from_str_radix(&(String::from_utf8_lossy(&[input[0]])
            + &*String::from_utf8_lossy(&[input[input.len() - 1]])), 16)? + 1;

        let real_chars = (input.len() - 2) as i32 / (key * 4);

        for _n in 0..real_chars {
            let mut hex_char = String::new();
            for _ in 0..4 {
                hex_char.push_str(&String::from_utf8_lossy(&[input[pos as usize]]));
                pos += key;
            }
            out.push_str(&String::from_utf8(decode(&hex_char)?)?);
        }
    }

    Ok(out)
}

