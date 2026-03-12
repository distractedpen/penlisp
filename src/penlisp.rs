pub mod tokenizer {
    use core::fmt;
    use std::char;

    #[derive(Debug, PartialEq)]
    pub enum Kind {
        Start,
        Keyword,
        Identifier,
        Integer,
        Decimal,
        Literal,
        End
    }

    impl fmt::Display for Kind {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Kind::Start => write!(f, "start"),
                Kind::End => write!(f, "end"),
                Kind::Keyword => write!(f, "keyword"),
                Kind::Literal => write!(f, "literal"),
                Kind::Integer => write!(f, "integer"),
                Kind::Decimal => write!(f, "decimal"),
                Kind::Identifier => write!(f, "identifier"),
            }
        }
    }

    static KEYWORDS: &[&'static str] = &[
        "eq", "add", "sub", "mul", "div", 
        "gt", "ge", "lt", "le", "neq",
        "and", "or", "not", "true", 
        "false", "if", "let", "defun", 
        "lambda", "cons", "nil" 
    ];

    fn get_kind(token: &String) -> Kind {
        let t = (*token).as_str();
        match t {
            "(" => Kind::Start,
            ")" => Kind::End,
            _ if KEYWORDS.contains(&t) => Kind::Keyword,
            _ if t.starts_with("\"") && t.ends_with("\"")=> Kind::Literal,
            _ if t.parse::<i64>().is_ok() => Kind::Integer,
            _ if t.parse::<f64>().is_ok() => Kind::Decimal,
            _ => {
                let t_chars: Vec<char> = t.chars().collect();
                if t_chars[0].is_ascii_alphabetic() || t_chars[0] == '_' {
                    return Kind::Identifier;
                }
                panic!("Cannot parse token {}", t);
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Token {
        pub loc: usize,
        pub kind: Kind,
        pub value: String,
    }

    impl PartialEq<Kind> for Token {
        fn eq(&self, other: &Kind) -> bool {
            self.kind == *other
        }
    }

    impl PartialEq<Token> for Kind {
        fn eq(&self, other: &Token) -> bool {
            *self == other.kind
        }
    }

    impl fmt::Display for Token {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Token{{loc: {}, kind: {}, value: {} }}", self.loc, self.kind, self.value)
        }
    }

    pub fn tokenize(data: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let chars: Vec<char> = data.chars().collect();
        let mut chars_index = 0;
        let mut token = String::new();
        while chars_index < chars.len() {
            match chars[chars_index] {
                ' ' | '\n' | '\t' => {
                    if !token.is_empty() {
                        tokens.push(Token{
                            loc: chars_index - token.len(),
                            kind: get_kind(&token),
                            value: token.clone()
                        });
                        token.clear();
                    }
                }
                '(' => {
                    tokens.push(Token{
                        loc: chars_index,
                        kind: Kind::Start,
                        value: String::from("("),
                    })
                }
                ')' => {
                    if !token.is_empty() {
                        tokens.push(Token{
                            loc: chars_index - token.len(),
                            kind: get_kind(&token),
                            value: token.clone()
                        });
                        token.clear();
                    }
                    tokens.push(Token{
                        loc: chars_index,
                        kind: Kind::End,
                        value: String::from(")"),
                    })
                }
                _ => {
                    token.push(chars[chars_index]);
                }
            }

            chars_index += 1;
        } tokens
    } 
}


pub fn run(data: String) {
    let tokens: Vec<tokenizer::Token> = tokenizer::tokenize(data);
    for token in tokens {
        print!("{}\n", token);
    }
}

// Graveyard:
// if let Some(t) = tokens.pop() {
//     match t.value.as_str() {
//         "+" => {
//             let lhs = Box::new(Expression::Number(tokens.pop().unwrap().value.parse::<i64>().unwrap()));
//             let rhs = Box::new(Expression::Number(tokens.pop().unwrap().value.parse::<i64>().unwrap()));
//
//             result.push(Expression::Add(lhs, rhs));
//         }
//         "-" => {
//             let lhs = Box::new(Expression::Number(tokens.pop().unwrap().value.parse::<i64>().unwrap()));
//             let rhs = Box::new(Expression::Number(tokens.pop().unwrap().value.parse::<i64>().unwrap()));
//
//             result.push(Expression::Subtract(lhs, rhs));
//         }
//         "*" => {
//             let lhs = Box::new(Expression::Number(tokens.pop().unwrap().value.parse::<i64>().unwrap()));
//             let rhs = Box::new(Expression::Number(tokens.pop().unwrap().value.parse::<i64>().unwrap()));
//
//             result.push(Expression::Mulitply(lhs, rhs));
//         }
//         "/" => {
//             let lhs = Box::new(Expression::Number(tokens.pop().unwrap().value.parse::<i64>().unwrap()));
//             let rhs = Box::new(Expression::Number(tokens.pop().unwrap().value.parse::<i64>().unwrap()));
//
//             result.push(Expression::Divide(lhs, rhs));
//         }
//         _ => unimplemented!("Blargg!!")
//     }
//
// pub mod interpreter {
//     use crate::penlisp::tokenizer;
//
//     #[derive(Debug)]
//     pub enum Expression {
//         Add(Box<Expression>, Box<Expression>),
//         Subtract(Box<Expression>, Box<Expression>),
//         Mulitply(Box<Expression>, Box<Expression>),
//         Divide(Box<Expression>, Box<Expression>),
//         Number(i64),
//         // Decimal(f64),
//         // Literal(String),
//         Nil,
//     }
//
//
//     // (+ 1 (* 3 4))
//     //      
//     pub fn process(tokens: &mut Vec<tokenizer::Token>) -> Vec<Expression> {
//         print!("{:?}\n", tokens);
//         if tokens.len() == 0 {
//             return vec![Expression::Nil];
//         }
//
//         let mut result: Vec<Expression> = vec![];
//         while tokens.len() > 0 {
//             let t = match tokens.pop() {
//                 Some(t) => t,
//                 None => panic!("No tokens left!!"),
//             };
//
//             match t.kind {
//                 tokenizer::Kind::Start => {
//                     let u = match tokens.pop() {
//                         Some(u) => u,
//                         None => panic!("No tokens left!!"),
//                     };
//
//                     if u.kind != tokenizer::Kind::Keyword {
//                         panic!("Expected Keyword after Start");
//                     }
//                 },
//                 tokenizer::Kind::End => {},
//                 tokenizer::Kind::Keyword => {},
//                 tokenizer::Kind::Value => {},
//             }
//         }
//         result
//     }
// }
