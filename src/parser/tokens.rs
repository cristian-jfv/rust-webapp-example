use std::collections::VecDeque;
use std::fmt;

use super::charstream::CharStream;
use super::errors::{ParserError, ParserResult};

#[derive(Debug, Clone)]
pub enum TokenKind {
    Plus,
    Minus,
    Times,
    Divide,
    OpenParenthesis,
    CloseParenthesis,
    Number,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Times => write!(f, "*"),
            TokenKind::Divide => write!(f, "/"),
            TokenKind::OpenParenthesis => write!(f, "("),
            TokenKind::CloseParenthesis => write!(f, ")"),
            TokenKind::Number => write!(f, "5"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: f64,
}

pub struct TokenStream {
    cs: CharStream,
    reserve_tokens: VecDeque<Token>,
}

impl TokenStream {
    pub fn new(input: String) -> Option<TokenStream> {
        let cs = match CharStream::new(input) {
            Ok(v) => v,
            Err(_) => {
                return None;
            }
        };
        Some(TokenStream {
            cs,
            reserve_tokens: VecDeque::new(),
        })
    }

    pub fn current_pos(&self) -> usize {
        return self.cs.current_pos();
    }

    pub fn get_token(&mut self) -> ParserResult<Option<Token>> {
        if self.reserve_tokens.is_empty() {
            //println!("token reserve empty, read new token");
            return Self::read_token(self);
        } else {
            match self.reserve_tokens.pop_back() {
                Some(t) => {
                    return Ok(Some(t));
                }
                None => {
                    return Err(ParserError::new(
                        self.cs.current_pos(),
                        "No tokens left to get".to_string(),
                    ))
                }
            }
        }
    }

    pub fn putback(&mut self, t: Token) {
        self.reserve_tokens.push_back(t.clone());
    }

    fn read_token(&mut self) -> ParserResult<Option<Token>> {
        loop {
            let ch = self.cs.next_char();
            match ch {
                None => {
                    return Ok(None); //Err(ParserError::new(self.cs.current_pos(), "".to_string()));
                }
                Some(' ') => continue, // Ignore whitespaces
                Some('+') => {
                    return Ok(Some(Token {
                        kind: TokenKind::Plus,
                        value: 0.0,
                    }))
                }
                Some('-') => {
                    return Ok(Some(Token {
                        kind: TokenKind::Minus,
                        value: 0.0,
                    }))
                }
                Some('*') => {
                    return Ok(Some(Token {
                        kind: TokenKind::Times,
                        value: 0.0,
                    }))
                }
                Some('/') => {
                    return Ok(Some(Token {
                        kind: TokenKind::Divide,
                        value: 0.0,
                    }))
                }
                Some('(') => {
                    return Ok(Some(Token {
                        kind: TokenKind::OpenParenthesis,
                        value: 0.0,
                    }))
                }
                Some(')') => {
                    return Ok(Some(Token {
                        kind: TokenKind::CloseParenthesis,
                        value: 0.0,
                    }))
                }
                Some(d) if d.is_numeric() => {
                    // Handle numerical tokens
                    self.cs.putback();
                    let number = Self::handle_numeric_tokens(&mut self.cs)?;
                    return Ok(Some(Token {
                        kind: TokenKind::Number,
                        value: number,
                    }));
                }
                Some(c) => {
                    return Err(ParserError::new(
                        self.cs.current_pos() + 1,
                        format!("Invalid character {}", c),
                    ));
                }
            }
        }
    }
    fn handle_numeric_tokens(cs: &mut CharStream) -> ParserResult<f64> {
        let mut number_str = String::new();
        // Read until end of number or end of string
        loop {
            match cs.next_char() {
                Some(c) => {
                    if c.is_numeric() || c == '.' || c == 'e' {
                        number_str.push(c)
                    } else if c == '-' && number_str.chars().nth_back(0) == Some('e') {
                        number_str.push(c)
                    } else {
                        cs.putback();
                        break;
                    }
                }
                None => break,
            }
        }

        // Try to parse the characters into a number
        match number_str.parse::<f64>() {
            Ok(number) => Ok(number),
            Err(_) => Err(ParserError::new(
                cs.current_pos(),
                "Error while reading number".to_string(),
            )),
        }
    }
}
