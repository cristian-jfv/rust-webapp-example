// Grammar proposed by B. Stroustrup in Programming Principles and Practice Using C++
// Expression:
//  Term
//  Expression "+" Term : addition
//  Expression "-" Term : substraction
// Term:
//  Primary
//  Term "*" Primary : multiplication
//  Term "/" Primary : division
//  Term "%" Primary : remainder (modulo)
// Primary:
//  Number
//  "(" Expression ")" : grouping
// Number:
//  floating-point-literal

pub mod charstream;
pub mod errors;
pub mod tokens;

use errors::{ParserError, ParserResult};
use tokens::{TokenKind, TokenStream};

const VERBOSE: bool = false;

fn log(message: String) {
    if VERBOSE {
        println!("{}", message);
    }
}

// Deal with + and -
pub fn expression(ts: &mut TokenStream) -> ParserResult<f64> {
    log("entering expression".to_string());
    let mut left = term(ts)?;

    loop {
        log("expression at the start of the loop".to_string());
        let t = ts.get_token()?;
        log("expression gets a valid token inside loop".to_string());
        let t = match t {
            Some(v) => v,
            None => break,
        };
        match t.kind {
            TokenKind::Plus => {
                log("expression finds a sum".to_string());
                let right = term(ts)?;
                left += right;
            }
            TokenKind::Minus => {
                log("expression finds a substraction".to_string());
                let right = term(ts)?;
                left -= right;
            }
            _ => {
                log("expression finds no match".to_string());
                ts.putback(t);
                break;
            }
        }
    }
    log(format!("expression returns {left}"));
    return Ok(left);
}

// Deal with *, /, and %
pub fn term(ts: &mut TokenStream) -> ParserResult<f64> {
    log("entering term".to_string());
    let mut left = primary(ts)?;
    loop {
        log("term at the start of the loop".to_string());
        let t = ts.get_token()?;
        log("term gets a valid token".to_string());
        let t = match t {
            Some(v) => v,
            None => break,
        };
        log(format!(
            "term: token.kind={}; token.value={}",
            t.kind, t.value
        ));
        match t.kind {
            TokenKind::Times => {
                log("term finds a multiplication".to_string());
                let right = primary(ts)?;
                log(format!("term: primary returns {}", right));
                left *= right;
            }
            TokenKind::Divide => {
                log("term finds a division".to_string());
                let d = primary(ts)?;
                if d == 0.0 {
                    eprintln!("Found division by zero");
                    break;
                }
                left /= d;
            }
            _ => {
                log("term finds no match".to_string());
                ts.putback(t);
                break;
            }
        }
    }
    log(format!("term returns {left}"));
    return Ok(left);
}

// Deal with numbers and parentheses
pub fn primary(ts: &mut TokenStream) -> ParserResult<f64> {
    log("entering primary".to_string());

    let t = ts.get_token()?;
    let t = match t {
        Some(v) => v,
        None => {
            return Err(ParserError::new(
                ts.current_pos(),
                "Expected token".to_string(),
            ));
        }
    };
    log(format!(
        "primary: token.kind={}; token.value={}",
        t.kind, t.value
    ));
    match t.kind {
        TokenKind::OpenParenthesis => {
            log("primary finds an opening parenthesis".to_string());
            let d = expression(ts)?;
            let t = ts.get_token()?;
            let t = match t {
                Some(v) => v,
                None => {
                    return Err(ParserError::new(
                        ts.current_pos(),
                        "Expected token".to_string(),
                    ));
                }
            };

            match t.kind {
                TokenKind::CloseParenthesis => {
                    log("primary finds a closing parenthesis".to_string());
                    return Ok(d);
                }
                _ => {
                    return Err(ParserError::new(
                        ts.current_pos(),
                        "Expected ')'".to_string(),
                    ));
                }
            }
        }
        TokenKind::Number => {
            log("primary finds a number".to_string());
            //println!("primary returns {}", t.value);
            return Ok(t.value);
        }
        _ => {
            //eprintln!("primary expected");
            return Err(ParserError::new(
                ts.current_pos(),
                "Primary expected".to_string(),
            ));
        }
    }
}
