use core::{fmt, panic};
use std::{char};


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Symbol {
    Lparen,
    Rparen,
    Identifier, 
    Integer,
    Decimal,
    Bool, // true, false
    Literal,
    Nil,
    Eq, Add, Sub, Mul, Div,
    Gt, Ge, Lt, Le, Ne,
    And, Or, Not, 
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
            Symbol::Bool => write!(f, "bool"),
            Symbol::Identifier => write!(f, "identifier"),
            Symbol::Nil => write!(f, "nil"),
            // Expressions
            Symbol::Add => write!(f, "+"),
            Symbol::Sub => write!(f, "-"),
            Symbol::Mul => write!(f, "*"),
            Symbol::Div => write!(f, "/"),
            // Conditions
            Symbol::Eq => write!(f, "="),
            Symbol::Gt => write!(f, ">"),
            Symbol::Ge => write!(f, ">="),
            Symbol::Lt => write!(f, "<"),
            Symbol::Le => write!(f, "<="),
            Symbol::Ne => write!(f, "!="),
            Symbol::And => write!(f, "and"),
            Symbol::Or => write!(f, "or"),
            Symbol::Not => write!(f, "not"),
        }
    }
}


fn get_symbol(token: &String) -> Symbol {
    let t = (*token).as_str();
    match t {
        "(" => Symbol::Lparen,
        ")" => Symbol::Rparen,
        "nil" => Symbol::Nil,
        "=" => Symbol::Eq,
        "+" => Symbol::Add,        
        "-" => Symbol::Sub,
        "*" => Symbol::Mul,
        "/" => Symbol::Div,
        ">" => Symbol::Gt,
        ">=" => Symbol::Ge,
        "<" => Symbol::Lt,
        "<=" => Symbol::Le,
        "!=" => Symbol::Ne,
        "and" => Symbol::And,
        "or" => Symbol::Or,
        "not" => Symbol::Not,
        "true" => Symbol::Bool,
        "false" => Symbol::Bool,
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


    fn binomial_op(&mut self, op: Symbol) -> Option<String> {
        
        let lhs_token: Token;
        if self.accept(Symbol::Integer) || self.accept(Symbol::Decimal) {
            lhs_token = self.lexer.consume_token().unwrap();
        } else if self.accept(Symbol::Lparen) {
            lhs_token = self.expression()
        } else {
            panic!("Unexpected token {}", self.lexer.peek_token().unwrap());
        }

        let rhs_token: Token;
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

    fn binomial_conditional_number_op(&mut self, op: Symbol) -> Option<String> {

        let lhs_token: Token;
        if self.accept(Symbol::Integer) || self.accept(Symbol::Decimal) {
            lhs_token = self.lexer.consume_token().unwrap();
        } else if self.accept(Symbol::Lparen) {
            lhs_token = self.expression();
        } else {
            return None;
        }

        let rhs_token: Token;
        if self.accept(Symbol::Integer) || self.accept(Symbol::Decimal) {
            rhs_token = self.lexer.consume_token().unwrap();
        } else if self.accept(Symbol::Lparen) {
            rhs_token = self.expression();
        } else {
            return None;
        }

        let lhs = lhs_token.value.parse::<f64>().unwrap();
        let rhs = rhs_token.value.parse::<f64>().unwrap();

        match op {
            Symbol::Gt => Some((lhs > rhs).to_string()),
            Symbol::Ge => Some((lhs >= rhs).to_string()),
            Symbol::Lt => Some((lhs < rhs).to_string()),
            Symbol::Le => Some((lhs <= rhs).to_string()),
            _ => None
        }
    }

    fn binomial_conditional_bool_op(&mut self, op: Symbol) -> Option<String> {

        let lhs_token: Token;
        if self.accept(Symbol::Bool) {
            lhs_token = self.lexer.consume_token().unwrap();
        } else if self.accept(Symbol::Lparen) {
            lhs_token = self.expression();
        } else {
            return None;
        }

        let rhs_token: Token;
        if self.accept(Symbol::Bool) {
            rhs_token = self.lexer.consume_token().unwrap();
        } else if self.accept(Symbol::Lparen) {
            rhs_token = self.expression();
        } else {
            return None;
        }

        let lhs = lhs_token.value.parse::<bool>().unwrap();
        let rhs = rhs_token.value.parse::<bool>().unwrap();

        match op {
            Symbol::And => Some((lhs && rhs).to_string()),
            Symbol::Or => Some((lhs || rhs).to_string()),
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
                    s @ Symbol::Add | // Binomial Operations on int and dec
                    s @ Symbol::Sub | 
                    s @ Symbol::Mul | 
                    s @ Symbol::Div => {
                        self.lexer.consume_token();
                        let v = match self.binomial_op(s) {
                            Some(v) => v,
                            None => panic!("Failed Binomial Op")
                        };

                        result = Token {
                            loc: t.loc,
                            symbol: Symbol::Integer,
                            value: v
                        };
                    }
                    // Cond
                    s @ Symbol::Ge | // can only take int and dec
                    s @ Symbol::Gt |
                    s @ Symbol::Lt |
                    s @ Symbol::Le => {
                        self.lexer.consume_token();
                        let b = match self.binomial_conditional_number_op(s) {
                            Some(b) => b,
                            None => panic!("Failed Conditional Op.")
                        };

                        result = Token {
                            loc: t.loc,
                            symbol: Symbol::Bool,
                            value: b
                        };
                    }
                    s @ Symbol::And |  // can only take bool
                    s @ Symbol::Or => {
                        self.lexer.consume_token();
                        let b = match self.binomial_conditional_bool_op(s) {
                            Some(b) => b,
                            None => panic!("Failed Conditional Op.")
                        };

                        result = Token {
                            loc: t.loc,
                            symbol: Symbol::Bool,
                            value: b
                        };
                    }
                    Symbol::Eq => {  //  can take either bool, int, dec, or string
                        unimplemented!("Eq not unimplemented")
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
