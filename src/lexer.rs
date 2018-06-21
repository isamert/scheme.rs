use std::iter::Peekable;
use std::str::Chars;

use util::GentleIterator;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Token {
    LParen,
    RParen,
    Symbol(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Chr(char),
    Str(String),
    Quote,
    //UnQuoteSugar
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let iter = &mut input.chars().peekable();

    loop {
        if parse_whitespace(iter) {
            continue
        }
            
        // or() is eagerly evaluated
        // thats why I used or_else
        let token = parse_lparen(iter)
            .or_else(|| parse_quote(iter))
            .or_else(|| parse_rparen(iter))
            .or_else(|| parse_string(iter))
            .or_else(|| parse_hash(iter))
            .or_else(|| parse_symbol(iter));

        if let Some(x) = token {
            tokens.push(x)
        } else {
            break;
        }
    };

    tokens
}

fn parse_whitespace(iter: &mut Peekable<Chars>) -> bool {
    if check_chr(iter, ' ') || check_chr(iter, '\n') {
        iter.next();
        true
    } else {
        false
    }
}

fn parse_quote(iter: &mut Peekable<Chars>) -> Option<Token> {
    if !check_chr(iter, '\'') {
        return None
    }

    iter.next();
    Some(Token::Quote)
}

fn parse_lparen(iter: &mut Peekable<Chars>) -> Option<Token> {
    if !check_chr(iter, '(') {
        return None
    }

    iter.next();
    Some(Token::LParen)
}

fn parse_rparen(iter: &mut Peekable<Chars>) -> Option<Token> {
    if !check_chr(iter, ')') {
        return None
    }

    iter.next();
    Some(Token::RParen)
}

fn parse_string(iter: &mut Peekable<Chars>) -> Option<Token> {
    if !check_chr(iter, '"') {
        return None
    }

    iter.next(); // Consume the opening "
    let value = iter
        .take_until(|c| *c != '"')
        .collect();
    iter.next(); // Consume the closing "
    Some(Token::Str(value))
}

fn parse_hash(iter: &mut Peekable<Chars>) -> Option<Token> {
    if !check_chr(iter, '#') {
        return None
    }

    iter.next(); // Consume #
    match iter.next() {
        Some('t') => Some(Token::Boolean(true)),  // #t means true
        Some('f') => Some(Token::Boolean(false)), // #f means false
        Some('\\') => {
            // #\a represents char 'a'
            // #\b represents char 'b'
            // ...
            let value = iter.next()
                .expect("Expected a char, got nothing.");
            Some(Token::Chr(value))
        },
        Some(c) => {
            panic!("Expected #t, #f or #\\<char> got: #{}", c)
        },
        None => {
            panic!("Expected something , got nothing: ....")
        } 
    }
}

fn parse_symbol(iter: &mut Peekable<Chars>) -> Option<Token> {
    // Check if iter is empty or not
    if !check(iter, |_| true) {
        return None
    }

    let value: String = iter
        .take_until(|c| *c != ' ' && *c != ')' && *c != '\n')
        .collect();

    if is_int(&value) {
        Some(Token::Integer(value.parse().unwrap()))
    } else if is_float(&value) {
        Some(Token::Float(value.parse().unwrap()))
    } else {
        Some(Token::Symbol(value))
    }
}

fn check<F>(iter: &mut Peekable<Chars>, fun: F) -> bool
where F: Fn(char) -> bool {
    if let Some(&x) = iter.peek() {
        fun(x)
    } else {
        false
    }
}

fn check_chr(iter: &mut Peekable<Chars>, chr: char) -> bool {
    check(iter, |x| x == chr)
}

fn is_int(x: &str) -> bool {
    x.chars()
        .all(char::is_numeric)
}

fn is_float(x: &str) -> bool {
    // Poor man's is_float
    x.chars()
        .all(|x| x.is_numeric() || x == '.')
        &&
        (x.len() > 1)
}
