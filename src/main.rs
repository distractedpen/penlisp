use std::{env::{self}, fmt::Debug, fs::{File}, io::{self, BufReader, Read, Write}};

#[derive(Debug)]
enum Atom {
    Literal(String),
    Number(i64),
    Add,
    Subtraction,
    Muliplication,
    Division,
}

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


fn interpreter(data: String) {
    // Parsing a singular S expression
    let tokens: Vec<String> = tokenize(data);
    print!("{:?}\n", tokens);
    let input_stack = process_tokens(tokens);
    print!("{:?}\n", input_stack);
    let result = process_stack(input_stack);
    match result {
        Ok(a) => {
            match a {
                Atom::Number(x)  => print!("{}\n", x),
                Atom::Literal(s) => print!("{}\n", s),
                _                => print!("{:?}", a),
            }
        },
        Err(s) => {
            println!("{}", s);
        }
    }
}


fn tokenize(data: String) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let chars: Vec<char> = data.chars().collect();
    let mut chars_index = 0;
    let mut token = String::new();
    while chars_index < chars.len() {
        match chars[chars_index] {
            ' ' | '\n' | '\t' => {
                if !token.is_empty() {
                    tokens.push(token.clone());
                    token.clear();
                }
            }
            '(' => {
                tokens.push(String::from("("));
            }
            ')' => {
                if !token.is_empty() {
                    tokens.push(token.clone());
                    token.clear();
                }
                tokens.push(String::from(")"));
            }
            _ => {
                token.push(chars[chars_index]);
            }
        }

        chars_index += 1;
    }

    tokens
}


fn process_tokens(tokens: Vec<String>) -> Vec<Atom> {
    let mut stack: Vec<Atom> = Vec::new();
    let mut index = 0;

    while index < tokens.len() {
        let token = &tokens[index];

        match token.as_str() {
            "(" => {
                index += 1;
                let action = tokens.get(index).expect("Malformed S-Expression.").as_str();
                match action {
                    "+" => {
                        stack.push(Atom::Add);
                    }
                    "-" => {
                        stack.push(Atom::Subtraction);
                    }
                    "*" => {
                        stack.push(Atom::Muliplication);
                    }
                    "/" => {
                        stack.push(Atom::Division);
                    }
                    _ => {
                        todo!("Unimplemented action");
                    }
                }
            }
            ")" => {
                // TODO:
                // Use this route to handle live interpreter parsing??
            }
            _ => {
                match token.parse::<i64>() {
                    Err(_) => stack.push(Atom::Literal(String::from(token))),
                    Ok(value) => stack.push(Atom::Number(value)),
                };
            }
        }

        index += 1;
    }

    stack
}


fn process_stack(mut input_stack: Vec<Atom>) -> Result<Atom, &'static str> {
    let mut process_stack: Vec<Atom> = vec![];
    while !input_stack.is_empty() {
        let atom = input_stack.pop().unwrap();

        match atom {
            Atom::Add => {
                let rhs = match process_stack.pop().unwrap() {
                    Atom::Number(x) => Some(x),
                    _ => None,
                }.unwrap();
                let lhs = match process_stack.pop().unwrap() {
                    Atom::Number(x) => Some(x),
                    _ => None,
                }.unwrap();
                process_stack.push(Atom::Number(rhs + lhs));
            },
            Atom::Subtraction => {
                let rhs = match process_stack.pop().unwrap() {
                    Atom::Number(x) => Some(x),
                    _ => None,
                }.unwrap();
                let lhs = match process_stack.pop().unwrap() {
                    Atom::Number(x) => Some(x),
                    _ => None,
                }.unwrap();
                process_stack.push(Atom::Number(rhs - lhs));
            },
            Atom::Muliplication => {
                let rhs = match process_stack.pop().unwrap() {
                    Atom::Number(x) => Some(x),
                    _ => None,
                }.unwrap();
                let lhs = match process_stack.pop().unwrap() {
                    Atom::Number(x) => Some(x),
                    _ => None,
                }.unwrap();
                process_stack.push(Atom::Number(rhs * lhs));
            },
            Atom::Division => {
                let top = match process_stack.pop().unwrap() {
                    Atom::Number(x) => Some(x),
                    _ => None,
                }.unwrap();
    

                let bot = match process_stack.pop().unwrap() {
                    Atom::Number(x) => Some(x),
                    _ => None,
                }.unwrap();

                if bot == 0 {
                    return Err("Error: Cannot divide by zero");
                }
                process_stack.push(Atom::Number(top / bot));
            },
            Atom::Number(x) => process_stack.push(Atom::Number(x)),
            Atom::Literal(s) => process_stack.push(Atom::Literal(s)),
        }
    }

    Ok(process_stack.pop().unwrap())
}

fn read_file(filename: String) -> std::io::Result<String>
{ let f = File::open(filename)?;
    let mut reader = BufReader::new(f);

    let mut data = String::new();
    reader.read_to_string(&mut data)?;

    Ok(data.trim().to_string())
}

fn shell_mode() -> std::io::Result<()> {
    let mut input = String::new();
    let prompt = "> ";
    print!("Starting Shell mode. Press Ctrl-C to exit.\n");
    loop {
        print!("{}", prompt);
        io::stdout().flush()?;
        let n = io::stdin().read_line(&mut input)?;
        if n == 0 {
            break
        } else {
            interpreter(input.clone());
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
            interpreter(data);
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
            interpreter(data);
        }
    }
}
