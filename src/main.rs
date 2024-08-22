use hex::decode;
use rand::prelude::*;
use std::io;
use std::io::Write;
use std::str;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    'outer: loop {
        print!("Input 1 for encryption, 2 for decryption, or 3 to exit: ");
        io::stdout().flush().unwrap();

        'inner: loop {
            let mut input = String::new();
            let _ = io::stdin().read_line(&mut input).expect("0");

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

                        match dec(&str.trim()) {
                            Ok(result) => {
                                print!("Decrypted text: {result}");

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
}

fn enc(input: &str, key: u8) -> String {
    let mut out = format!("{form:0>2}", form = format!("{key:x}")).to_string();
    let mut rng = thread_rng();

    let mut i = 0;
    for _n in input.graphemes(true) {
        let hex = format!("{:x}", input.trim().chars().nth(i).unwrap() as u8);
        i += 1;

        out = out + &*hex.graphemes(true).nth(0).unwrap().to_string();

        for _n in 0..key {
            out = out + &format!("{:x}", rng.gen_range(0..=15))
        }

        out = out + &*hex.chars().nth(1).unwrap().to_string();

        for _n in 0..key {
            out = out + &format!("{:x}", rng.gen_range(0..=15))
        }
    }

    out
}

fn dec(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let key = (i32::from_str_radix(&*(input.graphemes(true).nth(0).unwrap_or("").to_string()
        + input.graphemes(true).nth(1).unwrap_or("")), 16)?) + 1;


    let mut out = String::new();

    let mut pos: i32 = 2;

    let realchars = (input.graphemes(true).count() - 2) as i32 / (key * 2);

    for _n in 0..realchars {
        let mut hex = input.graphemes(true).nth(pos as usize).unwrap().to_owned();
        pos += key;
        hex += input.graphemes(true).nth(pos as usize).unwrap();
        pos += key;
        out += str::from_utf8(&*decode(hex).unwrap())?;
    }

    Ok(out)
}

