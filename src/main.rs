use std::io;
use std::io::Write;
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
                                println!("{}", enc(&text.trim(), k));

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
                    print!("Input string to encrypt: ");
                    io::stdout().flush().unwrap();

                    let mut str = String::new();
                    let _ = io::stdin().read_line(&mut str);

                    println!("{}", dec(&str.trim()));

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

    for n in 0..(input.graphemes(true).count() - 1) {
        let mut hex = String::new();
        let _ = input;
        match input.trim().chars().nth(n).parse::<u8>() {
            Ok(h) => {
                hex = format!("{h:x}");
            }
            _ => println!(":(")
        }
        out = out + hex
    }
    
    out
}

fn dec(input: &str) -> String {
    format!("Input: {input}").to_owned()
}
