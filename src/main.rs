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
        //cli_interface();
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
                    _ => {
                        println!("Something went wrong and I can't be bothered to figure out what it is");
                        process::exit(1);
                    }
                }
            }
            _ => {
                println!("Invalid key");
                process::exit(1);
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
            _ => {
                print!("Invalid input file");
                process::exit(1);
            }
        }
    } else {
        println!("No valid operation selected");
        process::exit(1);
    }
}

/*fn cli_interface() {
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

                        match key.trim().parse::<u8>() {
                            Ok(k) => {
                                println!("Encrypted text: {}", enc(&text.trim(), k));

                                break 'key_loop;
                            }
                            _ => {
                                print!("Please input a number from 0 to 255: ");
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
                        if str.trim().eq("C") || str.trim().eq("c") {
                            break 'dec_loop;
                        }

                        match dec(&str.trim(), "_") {
                            Ok(result) => {
                                println!("Decrypted text: {result}");

                                break 'dec_loop;
                            }
                            _ => {
                                print!("Invalid string, input C to cancel or input a valid string to continue: ");
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
}*/

fn enc(input: &str, in_path: &str, out_path: &str, key: u8) -> Result<String, Box<dyn Error>> {
    let file_input = fs::read(in_path)?;

    let mut out = format!("{form:0>2}", form = format!("{key:x}")).to_string();
    let mut rng = thread_rng();

    if !out_path.starts_with('_') {
        let mut file = File::create(out_path)?;

        for n in 0..String::from_utf8(file_input.clone()).unwrap().graphemes(true).count() {
            let hex_num = format!("{:x}", &file_input[n + 2]);
            let hex_num_v = format!("{:0>2}", hex_num).into_bytes();

            file.write_all((&String::from_utf8_lossy(&[hex_num_v[0]])).as_ref().as_ref())?;

            for _n in 0..key {
                file.write_all((&format!("{:x}", rng.gen_range(0..=15))).as_ref())?;
            }

            file.write_all((&String::from_utf8_lossy(&[hex_num_v[1]])).as_ref().as_ref())?;

            for _n in 0..key {
                file.write_all((&format!("{:x}", rng.gen_range(0..=15))).as_ref())?;
            }
        }
    } else if ! input.starts_with('_'){
        let input = input.to_owned().into_bytes();

        for n in 0..String::from_utf8(input.clone()).unwrap().graphemes(true).count() {
            let hex_num = format!("{:x}", input[n + 2]);
            let hex_num_v = format!("{:0>2}", hex_num).into_bytes();

            out.push_str(&String::from_utf8_lossy(&[hex_num_v[0]]));

            for _n in 0..key {
                out.push_str(&format!("{:x}", rng.gen_range(0..=15)));
            }

            out.push_str(&String::from_utf8_lossy(&[hex_num_v[1]]));

            for _n in 0..key {
                out.push_str(&format!("{:x}", rng.gen_range(0..=15)));
            }
        }
    }

    Ok(out)
}

fn dec(input: &str, in_path: &str, out_path: &str) -> Result<String, Box<dyn Error>> {
    let file_input = fs::read(in_path)?;
    let input = input.to_owned().into_bytes();

    let key = i32::from_str_radix(&(String::from_utf8_lossy(&[file_input[0]])
        + &*String::from_utf8_lossy(&[file_input[1]])), 16)? + 1;

    let mut out = String::new();

    let mut pos: i32 = 2;

    let real_chars = (file_input.iter().len() - 2) as i32 / (key * 2);

    if out_path.starts_with('_') {
        for _n in 0..real_chars {
            let mut hex_char = String::new();
            hex_char.push_str(&String::from_utf8_lossy(&[input[pos as usize]]));
            pos += key;
            hex_char.push_str(&String::from_utf8_lossy(&[input[pos as usize]]));
            pos += key;
            out.push_str(&String::from_utf8(decode(hex_char).unwrap())?);
        }
    } else {
        let mut file = File::create(out_path)?;
        //file.write_all(result.as_bytes())?;
        for _n in 0..real_chars {
            let mut hex_char = String::new();
            hex_char.push_str(&String::from_utf8_lossy(&[file_input[pos as usize]]));
            pos += key;
            hex_char.push_str(&String::from_utf8_lossy(&[file_input[pos as usize]]));
            pos += key;
            out.push_str(&String::from_utf8(decode(&hex_char).unwrap())?);
            file.write_all((&String::from_utf8(decode(&hex_char).unwrap())?).as_ref())?;
        }
    }

    Ok(out)
}

