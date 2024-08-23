use const_hex::decode;
use rand::prelude::*;
use std::{env, error::Error, fs, fs::File, io, io::Write, process, str};
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        let config = Config::build(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });

        //println!("Input path: {}\nOperation: {}\nKey: {}\nOutput path: {}", config.in_path, config.op, config.key, config.out_path);

        if let Err(e) = quiet(config) {
            println!("Application error: {e}");
            process::exit(1);
        }
    } else {
        cli_interface();
    }
}

struct Config {
    in_path: String,
    op: String,
    key: String,
    out_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }

        let in_path = args[1].clone();
        let op = args[2].clone();
        let key = args[3].clone();
        let out_path = args[4].clone();

        Ok(Config { in_path, op, key, out_path })
    }
}

fn quiet(config: Config) -> Result<String, Box<dyn Error>> {
    if config.op == "Encrypt" {
        match config.key.trim().parse::<u8>() {
            Ok(k) => {
                match enc("_", &config.in_path, &config.out_path, k) {
                    Ok(encrypted) => {
                        if config.out_path.starts_with("_") {
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
            }
            Err(error) => {
                println!("Invalid key, error: {error}");
                process::exit(0);
            }
        }
    } else if config.op == "Decrypt" {
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
        process::exit(1);
    }
}

fn cli_interface() {
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

    if ! in_path.starts_with('_') {
        let file_input = fs::read(in_path)?;

        if ! out_path.starts_with('_') {
            let mut out_file = File::create(out_path)?;
            out_file.write_all(out.as_bytes())?;

            for n in 0..String::from_utf8(file_input.clone()).unwrap().graphemes(true).count() {
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
            for n in 0..String::from_utf8(file_input.clone()).unwrap().graphemes(true).count() {
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
    } else if ! input.starts_with('_'){
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

    if ! in_path.starts_with('_') {
        let file_input = fs::read(in_path)?;

        let key = i32::from_str_radix(&(String::from_utf8_lossy(&[file_input[0]])
            + &*String::from_utf8_lossy(&[file_input[file_input.len() - 1]])), 16)? + 1;

        let real_chars = (file_input.len() - 2) as i32 / (key * 4);

        if ! out_path.starts_with('_') {
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
    } else if ! input.starts_with('_'){
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
            println!("{}", hex_char);
            out.push_str(&String::from_utf8(decode(&hex_char)?)?);
        }
    }

    Ok(out)
}

