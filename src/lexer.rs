#[derive(Debug, Eq, PartialEq)]
pub enum Token<'value> {
    Identifier(&'value str),
    Minus,
    Plus,
    Divide,
    Multiply,
    CompareEqual,
    CompareNotEqual,
    Equal,
    Return,
    If,
    Else,
    Func,
    Print,
    This,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    SemiColon,
    Arrow,
    Integer(i64),
}

pub struct Lexer;
impl Lexer {
    pub fn tokenize(input: &str) -> Vec<Token> {
        // Roughly estimate capacity
        let mut tokens = Vec::with_capacity(input.len() / 2);
        let chars = input.chars().collect::<Vec<char>>();
        let mut i = 0;
        while i < chars.len() {
            match chars[i] {
                _ if chars[i] == ' ' || chars[i] == '\n' || chars[i] == '\t' => {
                    i += 1;
                    continue;
                }
                '+' => tokens.push(Token::Plus),
                '-' => match chars.get(i + 1) {
                    Some(c) if c.eq(&'>') => {
                        tokens.push(Token::Arrow);
                        // skip the next char
                        i += 2;
                        continue;
                    }
                    _ => tokens.push(Token::Minus),
                },
                '*' => tokens.push(Token::Multiply),
                '/' => tokens.push(Token::Divide),
                '=' => match chars.get(i + 1) {
                    Some('=') => {
                        tokens.push(Token::CompareEqual);
                        i += 2;
                        continue;
                    }
                    Some('!') => {
                        tokens.push(Token::CompareNotEqual);
                        i += 2;
                        continue;
                    }
                    _ => tokens.push(Token::Equal),
                },
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                '{' => tokens.push(Token::LeftBracket),
                '}' => tokens.push(Token::RightBracket),
                ',' => tokens.push(Token::Comma),
                ':' => tokens.push(Token::Colon),
                ';' => tokens.push(Token::SemiColon),
                _ if chars[i].is_numeric() => {
                    let start = i;
                    while i < chars.len() && chars[i].is_numeric() {
                        i += 1;
                    }
                    let number = input[start..i].parse::<i64>().unwrap();
                    tokens.push(Token::Integer(number));
                    continue;
                }
                _ if chars[i].is_alphabetic() => {
                    let start = i;
                    while i < chars.len() && chars[i].is_alphabetic() {
                        i += 1;
                    }
                    let new_string = &input[start..i];
                    match new_string {
                        "if" => tokens.push(Token::If),
                        "else" => tokens.push(Token::Else),
                        "fn" => tokens.push(Token::Func),
                        "print" => tokens.push(Token::Print),
                        "return" => tokens.push(Token::Return),
                        "this" => tokens.push(Token::This),
                        _ => tokens.push(Token::Identifier(new_string)),
                    }
                    continue;
                }
                _ => {
                    panic!("Unexpected character {} at position: {} ", chars[i], i);
                }
            }
            i += 1;
        }

        tokens
    }
}
