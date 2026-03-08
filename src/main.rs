use std::{env::{self}, fmt::Debug, fs::File, io::{BufReader, Read}};

#[derive(Debug)]
enum Atom {
    Literal(String),
    Number(i64),
    Add,
    Subtraction,
    Muliplication,
    Division,
}


fn read_file(filename: String) -> std::io::Result<String>
{ let f = File::open(filename)?;
    let mut reader = BufReader::new(f);

    let mut data = String::new();
    reader.read_to_string(&mut data)?;

    Ok(data.trim().to_string())
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


fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();

    let data = read_file(filename).expect("Failed to Read input file.");

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
