use core::{fmt, panic};
use std::{char};


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Symbol {
    Lparen,
    Rparen,
    Identifier, 
    Integer,
    Decimal,
    Literal,
    Nil,
    Eq, Add, Sub, Mul, Div,
    Gt, Ge, Lt, Le, Ne,
    And, Or, Not, True, False,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Sentinals
            Symbol::Lparen => write!(f, "lparen"),
            Symbol::Rparen => write!(f, "rparen"),
            // Types
            Symbol::Literal => write!(f, "literal"),
            Symbol::Integer => write!(f, "integer"),
            Symbol::Decimal => write!(f, "decimal"),
            Symbol::Identifier => write!(f, "identifier"),
            Symbol::Nil => write!(f, "nil"),
            // Expressions
            Symbol::Add => write!(f, "add"),
            Symbol::Sub => write!(f, "sub"),
            Symbol::Mul => write!(f, "mul"),
            Symbol::Div => write!(f, "div"),
            // Conditions
            Symbol::Eq => write!(f, "eq"),
            Symbol::Gt => write!(f, "gt"),
            Symbol::Ge => write!(f, "ge"),
            Symbol::Lt => write!(f, "lt"),
            Symbol::Le => write!(f, "le"),
            Symbol::Ne => write!(f, "ne"),
            Symbol::And => write!(f, "and"),
            Symbol::Or => write!(f, "or"),
            Symbol::Not => write!(f, "not"),
            Symbol::True => write!(f, "true"),
            Symbol::False => write!(f, "false"),

        }
    }
}


fn get_symbol(token: &String) -> Symbol {
    let t = (*token).as_str();
    match t {
        "(" => Symbol::Lparen,
        ")" => Symbol::Rparen,
        "nil" => Symbol::Nil,
        "eq" => Symbol::Eq,
        "add" => Symbol::Add,        
        "sub" => Symbol::Sub,
        "mul" => Symbol::Mul,
        "div" => Symbol::Div,
        "gt" => Symbol::Gt,
        "ge" => Symbol::Ge,
        "lt" => Symbol::Lt,
        "le" => Symbol::Le,
        "ne" => Symbol::Ne,
        "and" => Symbol::And,
        "or" => Symbol::Or,
        "not" => Symbol::Not,
        "true" => Symbol::True,
        "false" => Symbol::False,
        _ if t.starts_with("\"") && t.ends_with("\"")=> Symbol::Literal,
        _ if t.parse::<i64>().is_ok() => Symbol::Integer,
        _ if t.parse::<f64>().is_ok() => Symbol::Decimal,
        _ => {
            let t_chars: Vec<char> = t.chars().collect();
            if t_chars[0].is_ascii_alphabetic() || t_chars[0] == '_' {
                return Symbol::Identifier;
            }
            panic!("Cannot parse token {}", t);
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub loc: usize,
    pub symbol: Symbol,
    pub value: String,
}

impl Token {

    pub fn nil(loc: usize) -> Self {
        Token {
            loc: loc,
            symbol: Symbol::Nil,
            value: "nil".to_string()
        }
    }
}

impl PartialEq<Symbol> for Token {
    fn eq(&self, other: &Symbol) -> bool {
        self.symbol == *other
    }
}

impl PartialEq<Token> for Symbol {
    fn eq(&self, other: &Token) -> bool {
        *self == other.symbol
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token{{loc: {}, symbol: {}, value: {} }}", self.loc, self.symbol, self.value)
    }
}

#[derive(Debug)]
pub struct Lexer 
{
    tokens: Vec<Token>
}

impl Lexer {

    pub fn debug_print(&self) {
        for i in 0..self.tokens.len() {
            print!("\t{}\n", self.tokens[self.tokens.len() - 1 - i]);
        }
        println!();
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn peek_token(&self) -> Option<Token> {
        if self.tokens.len() == 0 {
            return None
        }
        return Some(self.tokens[self.tokens.len() - 1].clone());
    }

    pub fn consume_token(&mut self) -> Option<Token> {
        return self.tokens.pop();
    }

    pub fn error(&self, message: &str) {
        println!("{}", message);
    }

    pub fn tokenize(data: String) -> Self {
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
                            symbol: get_symbol(&token),
                            value: token.clone()
                        });
                        token.clear();
                    }
                }
                '(' => {
                    tokens.push(Token{
                        loc: chars_index,
                        symbol: Symbol::Lparen,
                        value: String::from("("),
                    })
                }
                ')' => {
                    if !token.is_empty() {
                        tokens.push(Token{
                            loc: chars_index - token.len(),
                            symbol: get_symbol(&token),
                            value: token.clone()
                        });
                        token.clear();
                    }
                    tokens.push(Token{
                        loc: chars_index,
                        symbol: Symbol::Rparen,
                        value: String::from(")"),
                    })
                }
                _ => {
                    token.push(chars[chars_index]);
                }
            }
            chars_index += 1;
        } 
        
        tokens.reverse();
        Lexer { tokens }
    } 
}


#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
}

impl Parser {

    fn accept(&self, accepted: Symbol) -> bool {
        match self.lexer.peek_token() {
            Some(t) => t.symbol == accepted,
            None => panic!("No symbols left")
        }
    }

    fn expect(&self, expected: Symbol) -> bool {
        if self.accept(expected) {
            return true;
        }
        self.lexer.error("Unexpected symbol!");
        return false;
    }

    // // assumes the next token exists and is an integer
    // fn integer(&mut self) -> i64 {
    //     let t = self.lexer.consume_token().unwrap();
    //     t.value.parse::<i64>().unwrap()
    // }
    //
    // // assumes the next token exists and is an decimal
    // fn decimal(&mut self) -> f64 {
    //     let t = self.lexer.consume_token().unwrap();
    //     t.value.parse::<f64>().unwrap()
    // }

    fn binomial_op(&mut self, op: Symbol) -> Option<String> {
        
        let lhs_token: Token;
        let rhs_token: Token;
        if self.accept(Symbol::Integer) || self.accept(Symbol::Decimal) {
            lhs_token = self.lexer.consume_token().unwrap();
        } else if self.accept(Symbol::Lparen) {
            lhs_token = self.expression()
        } else {
            panic!("Unexpected token {}", self.lexer.peek_token().unwrap());
        }

        if self.accept(Symbol::Integer) || self.accept(Symbol::Decimal) {
            rhs_token = self.lexer.consume_token().unwrap();
        } else if self.accept(Symbol::Lparen) {
            rhs_token = self.expression()
        } else {
            panic!("Unexpected token {}", self.lexer.peek_token().unwrap());
        }

        match (lhs_token.symbol, rhs_token.symbol) {
            (Symbol::Integer, Symbol::Integer) => {
                let lhs = lhs_token.value.parse::<i64>().unwrap();
                let rhs = rhs_token.value.parse::<i64>().unwrap();
                match op {
                    Symbol::Add => Some((lhs + rhs).to_string()),
                    Symbol::Sub => Some((lhs - rhs).to_string()),
                    Symbol::Mul => Some((lhs * rhs).to_string()),
                    Symbol::Div => {
                        if rhs == 0 {
                            panic!("Divide by zero");
                        }
                        Some((lhs / rhs).to_string())
                    }
                    _ => None
                }
            },
            (Symbol::Decimal, Symbol::Integer) | 
                (Symbol::Integer, Symbol::Decimal) |
                (Symbol::Decimal, Symbol::Decimal) => {
                let lhs = lhs_token.value.parse::<f64>().unwrap();
                let rhs = rhs_token.value.parse::<f64>().unwrap();
                match op {
                    Symbol::Add => Some((lhs + rhs).to_string()),
                    Symbol::Sub => Some((lhs - rhs).to_string()),
                    Symbol::Mul => Some((lhs * rhs).to_string()),
                    Symbol::Div => {
                        if rhs == 0.0 {
                            panic!("Divide by zero");
                        }
                        Some((lhs / rhs).to_string())
                    }
                    _ => None
                }
            },
            _ => None
        }
    }


    fn expression(&mut self) -> Token {
        if self.expect(Symbol::Lparen) {
            self.lexer.consume_token();
        } else {
            panic!("Expected (")
        }

        let result: Token;
        
        match self.lexer.peek_token() {
            Some(t) => {
                match t.symbol {
                    Symbol::Add => {
                        self.lexer.consume_token();
                        let v = match self.binomial_op(Symbol::Add) {
                            Some(v) => v,
                            None => panic!("What the hell?")
                        };

                        result = Token {
                            loc: t.loc,
                            symbol: Symbol::Integer,
                            value: v
                        };
                    }
                    Symbol::Sub => {
                        self.lexer.consume_token();
                        let v = match self.binomial_op(Symbol::Sub) {
                            Some(v) => v,
                            None => panic!("Invalid input!!")
                        };

                        result = Token {
                            loc: t.loc,
                            symbol: Symbol::Integer,
                            value: v
                        };
                    }
                    Symbol::Mul => {
                        self.lexer.consume_token();
                        let v = match self.binomial_op(Symbol::Mul) {
                            Some(v) => v,
                            None => panic!("Invalid input!!")
                        };

                        result = Token {
                            loc: t.loc,
                            symbol: Symbol::Integer,
                            value: v
                        };
                    }
                    Symbol::Div => {
                        self.lexer.consume_token();
                        let v = match self.binomial_op(Symbol::Div) {
                            Some(v) => v,
                            None => {
                                return Token::nil(t.loc);
                            }
                        };

                        result = Token {
                            loc: t.loc,
                            symbol: Symbol::Integer,
                            value: v
                        };
                    }
                    _ => {
                        panic!("Unexpected or unimlemented symbol {}", t);
                    }
                }
            },
            None => {
                panic!("Invalid Syntax");
            }
        }
        if self.expect(Symbol::Rparen) {
            self.lexer.consume_token();
        } else {
            panic!("Expected )");
        }

        result
    }

    fn run(&mut self) {
        let result = self.expression();
        print!("{}\n", result.value);
    }


    fn new(lexer: Lexer) -> Self {
        Parser { lexer: lexer }
    }
}

pub fn run(data: String) {
    let lexer: Lexer = Lexer::tokenize(data);
    Parser::new(lexer).run();
}
