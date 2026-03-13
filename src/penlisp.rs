use core::fmt;
use std::char;

#[derive(Debug, Copy, Clone, PartialEq)]
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

pub enum Keyword {
    Eq,
    Add,
    Sub,
    Mul,
    Div,
    Gt,
    Ge,
    Lt,
    Le,
    Neq,
    And,
    Or,
    Not,
    True,
    False,
    If,
    Let,
    Defun,
    Lambda,
    Cons,
    Nil 
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
                Keyword::Eq => write!(f, "eq"),
                Keyword::Add => write!(f, "add"),
                Keyword::Sub => write!(f, "sub"),
                Keyword::Mul => write!(f, "mul"),
                Keyword::Div => write!(f, "div"),
                Keyword::Gt => write!(f, "gt"),
                Keyword::Ge => write!(f, "ge"),
                Keyword::Lt => write!(f, "lt"),
                Keyword::Le => write!(f, "le"),
                Keyword::Neq => write!(f, "neq"),
                Keyword::And => write!(f, "and"),
                Keyword::Or => write!(f, "or"),
                Keyword::Not => write!(f, "not"),
                Keyword::True => write!(f, "true"),
                Keyword::False => write!(f, "false"),
                Keyword::If => write!(f, "if"),
                Keyword::Let => write!(f, "let"),
                Keyword::Defun => write!(f, "defun"),
                Keyword::Lambda => write!(f, "lambda"),
                Keyword::Cons => write!(f, "cons"),
                Keyword::Nil => write!(f, "nil"),
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


#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug)]
pub struct Lexer 
{
    tokens: Vec<Token>
}

impl Lexer {

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn peek_token(&self) -> Option<Token> {
        if self.tokens.len() > 0 {
            return None
        }
        return Some(self.tokens[self.tokens.len() - 1].clone());
    }

    pub fn consume_token(&mut self) -> Option<Token> {
        return self.tokens.pop();
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
        } 

        Lexer { tokens }
    } 
}


pub mod parser {
    /***
    * # program: start of program
    * # expression: a keyword or identifier followed by any number of terms
    * # term: number, string literal, boolean value (true, false)
    *
    *
    * program = expression
    * expression  =
    *     [ "(" keyword expresson { expression } ")" ]
    *     [ "(" ")" ]
    ***/
}



pub fn run(data: String) {
    let mut lexer: Lexer = Lexer::tokenize(data);
    while lexer.len() > 0 {
        print!("{}\n", lexer.consume_token().unwrap());
    }
}
