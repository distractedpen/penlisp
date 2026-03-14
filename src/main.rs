use std::{env::{self}, fs::{File}, io::{BufReader, Read, Write}};

mod penlisp;

struct CliArgs {
    shell_mode: bool,
    filename: String,
}

fn parse_args() -> Result<CliArgs, String> {
    let raw_args: Vec<String> = env::args().collect();
    let mut shell_mode = true;
    let mut filename = String::new();

    let mut index = 1;
    while index < raw_args.len() {

        match raw_args.get(index) {
            Some(a) => {
                match (*a).as_str() {
                    "--noshell" => {
                        shell_mode = false;
                    },
                    "--file" | "-f" => {
                        filename = match raw_args.get(index + 1) {
                            Some(arg) => (*arg).clone(),
                            None => {
                                return Err("Missing filename for file flag.".to_string())
                            }
                        };
                        index += 1;
                    },
                    _ => {
                        return Err(format!("Unknown flag: {}", *a));
                    },
                }
            }
            None => {
                return Err("How did we get an empty in the args?".to_string());
            }
        };

        index += 1;
    }

    match raw_args.iter().position(|f| f == "-f" || f == "--file") {
        Some(index) => {
            if index + 1 > raw_args.len() {
                return Err("Error: filename not provided.".to_string());
            }
            filename = raw_args[index + 1].to_string();
        },
        None => {},
    }

    Ok(CliArgs {
        shell_mode,
        filename,
    })

}


fn read_file(filename: String) -> std::io::Result<String>
{ let f = File::open(filename)?;
    let mut reader = BufReader::new(f);

    let mut data = String::new();
    reader.read_to_string(&mut data)?;

    Ok(data.trim().to_string())
}

pub fn shell_mode() -> std::io::Result<()> {
    let mut input = String::new();
    let prompt = "> ";
    print!("Starting Shell mode. Press Ctrl-C to exit.\n");
    loop {
        print!("{}", prompt);
        std::io::stdout().flush()?;
        let n = std::io::stdin().read_line(&mut input)?;
        if n == 0 {
            break
        } else {
            penlisp::run(input.trim().to_string().clone());
        }
        input.clear();
    }

    Ok(())
}


fn main() {

    let args = match parse_args() {
        Ok(a) => a,
        Err(error) => {
            print!("{}", error);
            return;
        }
    };
    if args.shell_mode {
        if !args.filename.is_empty() {
            let data = read_file(args.filename).expect("Failed to Read input file.");
            penlisp::run(data);
        }
        match shell_mode() {
            Ok(_) => {
                println!("Exiting...");
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    } else {
        if !args.filename.is_empty() {
            let data = read_file(args.filename).expect("Failed to Read input file.");
            penlisp::run(data);
        }
    }
}
