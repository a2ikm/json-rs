use crate::tokenize::{Token, tokenize};
use crate::{Error, Result};
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug, PartialEq)]
pub enum Value {
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

pub fn parse(source: &str) -> Result<Value> {
    let tokens = tokenize(source)?;
    let mut tokens = tokens.iter().peekable();
    let result = parse_value(&mut tokens);

    if let Some(token) = tokens.peek() {
        return Err(Error::UnexpectedToken(token.to_string()));
    }

    result
}

fn parse_value(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Value> {
    if let Some(token) = tokens.next() {
        match token {
            Token::Null => Ok(Value::Null),
            Token::True => Ok(Value::Bool(true)),
            Token::False => Ok(Value::Bool(false)),
            Token::Number(value) => Ok(Value::Number(*value)),
            Token::String(string) => Ok(Value::String(string.clone())),
            Token::LeftSquareBracket => parse_array(tokens),
            Token::LeftCurlyBracket => parse_object(tokens),
            _ => Err(Error::UnexpectedToken(token.to_string())),
        }
    } else {
        Err(Error::UnexpectedEOF)
    }
}

fn parse_array(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Value> {
    let mut array = Vec::new();

    match tokens.peek() {
        Some(Token::RightSquareBracket) => {
            tokens.next(); // consume bracket
            return Ok(Value::Array(array));
        }
        _ => {
            let value = parse_value(tokens)?;
            array.push(value);
        }
    }

    loop {
        if let Some(&token) = tokens.peek() {
            match token {
                Token::RightSquareBracket => {
                    tokens.next(); // consume bracket
                    return Ok(Value::Array(array));
                }
                Token::Comma => {
                    tokens.next(); // consume comma
                    let value = parse_value(tokens)?;
                    array.push(value);
                }
                _ => {
                    return Err(Error::UnexpectedToken(token.to_string()));
                }
            }
        } else {
            return Err(Error::UnexpectedEOF);
        }
    }
}

fn parse_object(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Value> {
    let mut hash = HashMap::new();

    match tokens.peek() {
        Some(Token::RightCurlyBracket) => {
            tokens.next(); // consume bracket
            return Ok(Value::Object(hash));
        }
        _ => {
            let (key, value) = parse_key_value_pair(tokens)?;
            hash.insert(key, value);
        }
    }

    loop {
        if let Some(&token) = tokens.peek() {
            match token {
                Token::RightCurlyBracket => {
                    tokens.next(); // consume bracket
                    return Ok(Value::Object(hash));
                }
                Token::Comma => {
                    tokens.next(); // consume comma
                    let (key, value) = parse_key_value_pair(tokens)?;
                    hash.insert(key, value);
                }
                _ => {
                    return Err(Error::UnexpectedToken(token.to_string()));
                }
            }
        } else {
            return Err(Error::UnexpectedEOF);
        }
    }
}

fn parse_key_value_pair(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<(String, Value)> {
    let key = match tokens.next() {
        Some(Token::String(string)) => string.clone(),
        Some(token) => return Err(Error::UnexpectedToken(token.to_string())),
        None => return Err(Error::UnexpectedEOF),
    };

    match tokens.next() {
        Some(Token::Colon) => (),
        Some(token) => return Err(Error::UnexpectedToken(token.to_string())),
        None => return Err(Error::UnexpectedEOF),
    }

    let value = parse_value(tokens)?;

    Ok((key, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_null() {
        assert_eq!(parse("null"), Ok(Value::Null));
    }

    #[test]
    fn parse_true() {
        assert_eq!(parse("true"), Ok(Value::Bool(true)));
    }

    #[test]
    fn parse_false() {
        assert_eq!(parse("false"), Ok(Value::Bool(false)));
    }

    #[test]
    fn parse_number() {
        assert_eq!(parse("-123.45E10"), Ok(Value::Number(-123.45e10)));
    }

    #[test]
    fn parse_string() {
        assert_eq!(parse("\"foo\""), Ok(Value::String("foo".to_string())));
    }

    #[test]
    fn parse_array() {
        assert_eq!(
            parse("[1,2,3]"),
            Ok(Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0)
            ]))
        );
    }

    #[test]
    fn parse_array_in_array() {
        assert_eq!(
            parse("[[1],[2],[3]]"),
            Ok(Value::Array(vec![
                Value::Array(vec![Value::Number(1.0)]),
                Value::Array(vec![Value::Number(2.0)]),
                Value::Array(vec![Value::Number(3.0)])
            ]))
        );
    }

    #[test]
    fn parse_object() {
        assert_eq!(
            parse("{\"foo\":\"bar\"}"),
            Ok(Value::Object(HashMap::from([(
                "foo".to_string(),
                Value::String("bar".to_string())
            )])))
        );
    }

    #[test]
    fn parse_error_trailing_token() {
        assert_eq!(parse("{},"), Err(Error::UnexpectedToken(",".to_string())));
    }
}
