
#[derive(Debug)]
pub enum Token {
    Ident(String),
    StringLit(String),
    IntLit(i32),
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    DEquals,
    If,
    ColonColon,
}

pub fn lex(src: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = src.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            '{' => tokens.push(Token::OpenBrace),
            '}' => tokens.push(Token::CloseBrace),
            '=' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::DEquals);
                }
            }

            ':' => {
                if chars.peek() == Some(&':') {
                    chars.next();
                    tokens.push(Token::ColonColon);
                }
            }

            '"' => {
                let mut value = String::new();

                while let Some(&next) = chars.peek() {
                    chars.next();

                    if next == '"' {
                        break;
                    }

                    value.push(next);
                }

                tokens.push(Token::StringLit(value));
            }

            c if c.is_whitespace() => continue,

            c if c.is_ascii_digit() => {
                let mut num = (c as u8 - b'0') as i32;
                while let Some(&next) = chars.peek() {
                    if !next.is_ascii_digit() {break;}
                    chars.next();
                    num = num * 10 + (next as u8 - b'0') as i32;
                }
                tokens.push(Token::IntLit(num))
            }

            c if c.is_ascii_alphabetic() => {
                let mut ident = String::new();
                ident.push(c);
                while let Some(&next) = chars.peek() {
                    if next.is_ascii_alphanumeric() || next == '_' {
                        chars.next();
                        ident.push(next);
                    } else {break};
                }
                let token = match ident.as_str() {
                    "if" => Token::If,
                    _ => Token::Ident(ident)
                };
                tokens.push(token);
            }
            _ => {}
        }
    }
    tokens
}
