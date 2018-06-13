use std::iter::Peekable;
use std::vec::IntoIter;

use lexer::Token;
use procedure::ProcedureData;


#[derive(Debug, Clone)]
pub enum SExpr {
    Atom(Token),
    List(Vec<SExpr>),
    Procedure(ProcedureData)
}

pub fn parse(tokens: Vec<Token>) -> Vec<SExpr> {
    let mut iter = tokens.into_iter().peekable();
    let mut exprs: Vec<SExpr> = vec![];

    while let Some(_) = iter.peek() {
        exprs.push(parse_helper(&mut iter));
    }

    exprs
}

fn parse_helper(iter: &mut Peekable<IntoIter<Token>>) -> SExpr {
    match iter.peek() {
        Some(&Token::RParen) => panic!("Not expected a )."),
        Some(&Token::LParen) => {
            iter.next(); // Consume LParen
            let mut list: Vec<SExpr> = vec![];
            while iter.peek() != Some(&Token::RParen) {
                list.push(parse_helper(iter));
            }
            iter.next(); // Consume RParen
            SExpr::List(list)
        },
        Some(_) => { 
            let y = iter.next().unwrap(); 
            SExpr::Atom(y) 
        },
        None => panic!("Expected a token, found none."),
    }
}

