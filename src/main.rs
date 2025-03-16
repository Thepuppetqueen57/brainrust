use std::io::Write;
use std::process::exit;
use std::fs;

fn parse(code: String) {
    let mut byte_array: Vec<i16> = vec![0];
    let mut index: u16 = 0;

    for character in code.chars() {
        match character {
            '+' => {
                byte_array[index as usize] += 1
            }

            '-' => {
                byte_array[index as usize] -= 1
            }

            '>' => {
                if byte_array.len() > index as usize {
                    byte_array.push(0);
                }

                index += 1
            }

            '<' => {
                index -= 1
            }

            '*' => {
                println!("{}", byte_array[index as usize]);
            }

            '.' => {
                print!("{}", (byte_array[index as usize] as u8 % 128) as char);
                std::io::stdout().flush().unwrap();
            }

            ',' => {
                let mut cmd: String = Default::default();

                let _ = std::io::stdin().read_line(&mut cmd).unwrap();
                cmd = cmd.trim().to_string();

                match cmd.parse::<i16>() {
                    Ok(cmdint) => {
                        byte_array[index as usize] = cmdint;
                    }

                    _ => {
                        println!("Error 1: Failed to convert input to an int!");
                        exit(1);
                    }
                }
            }

            _ => {}
        }
    }
}

fn main() {
    let mut cliargs: Vec<String> = vec![];

    for argument in std::env::args().skip(1) {
        cliargs.push(argument);
    }

    if cliargs.is_empty() {
        println!("You didn't supply arguments!");
        exit(1)
    }

    if cliargs[0] == "file" {
        let read_result = fs::read_to_string(&cliargs[1]);

        match read_result {
            Ok(code) => {
                parse(code);
            }

            _ => {
                println!("Failed to read file!");
                println!("Does it exist?");
                exit(1);
            }
        }
    } else if cliargs[0] == "text" {
        parse(cliargs[1].clone());
    }
}