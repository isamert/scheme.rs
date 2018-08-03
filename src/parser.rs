use std::iter::Peekable;
use std::vec::IntoIter;

use lexer::Token;
use procedure::ProcedureData;
use evaluator;
use env::EnvRef;


#[derive(Debug, Clone)]
pub enum SExpr {
    Atom(Token),
    List(Vec<SExpr>),
    Pair(Box<(SExpr, SExpr)>),
    Procedure(ProcedureData),
    Lazy(Box<SExpr>),
    Unspecified
}

#[allow(dead_code)]
impl SExpr {
    pub fn symbol(x: &str) -> SExpr {
        SExpr::Atom(Token::Symbol(x.to_string()))
    }

    pub fn integer(x: i64) -> SExpr {
        SExpr::Atom(Token::Integer(x))
    }

    pub fn float(x: f64) -> SExpr {
        SExpr::Atom(Token::Float(x))
    }

    pub fn boolean(x: bool) -> SExpr {
        SExpr::Atom(Token::Boolean(x))
    }

    pub fn chr(x: char) -> SExpr {
        SExpr::Atom(Token::Chr(x))
    }

    pub fn str_(x: &str) -> SExpr {
        SExpr::Atom(Token::Str(x.to_string()))
    }

    pub fn quasiquote(mut args: Vec<SExpr>) -> SExpr {
        args.insert(0, SExpr::symbol("quasiquote"));
        SExpr::List(args)
    }

    pub fn quote(sexpr: SExpr) -> SExpr {
        SExpr::List(vec![SExpr::symbol("quote"), sexpr])
    }

    pub fn unquote(sexpr: SExpr) -> SExpr {
        SExpr::List(vec![SExpr::symbol("unquote"), sexpr])
    }

    pub fn lazy(x: SExpr) -> SExpr {
        SExpr::Lazy(Box::new(x))
    }

    pub fn to_bool(&self) -> bool {
        // Anything other than #f is treated as true.
        match self {
            SExpr::Atom(Token::Boolean(x)) => x.clone(),
            _ => true
        }
    }

    pub fn is_symbol(&self, symbol: &str) -> bool {
        match self {
            SExpr::Atom(Token::Symbol(x)) => x == symbol,
            _ => false
        }
    }

    pub fn is_lazy(&self) -> bool {
        match self {
            SExpr::Lazy(_) => true,
            _ => false
        }
    }

    pub fn into_symbol(self) -> Option<String> {
        match self {
            SExpr::Atom(Token::Symbol(x)) => Some(x),
            _ => None
        }
    }

    pub fn into_list(self) -> Option<Vec<SExpr>> {
        match self {
            SExpr::List(x) => Some(x),
            _ => None
        }
    }

    pub fn eval(&self, env: &EnvRef) -> SExpr {
        evaluator::eval(self, env)
    }
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

            // Check if empty list
            if iter.peek() == Some(&Token::RParen) {
                iter.next(); // Consume RParen
                return SExpr::List(vec![]);
            }

            let head = parse_helper(iter);
            let dotted = iter.peek() == Some(&Token::Symbol(".".to_string()));

            let result = if dotted {
                iter.next(); // Consume '.'
                let tail = parse_helper(iter);
                SExpr::Pair(Box::new((head, tail)))
            } else {
                let mut tail: Vec<SExpr> = vec![];
                while iter.peek() != Some(&Token::RParen) {
                    tail.push(parse_helper(iter));
                }

                tail.insert(0, head);
                SExpr::List(tail)
            };

            iter.next(); // Consume RParen
            result
        },
        Some(&Token::Quote) => {
            iter.next();
            SExpr::List(vec![SExpr::symbol("quote"), parse_helper(iter)])
        },
        Some(&Token::UnQuote) => {
            iter.next();
            SExpr::List(vec![SExpr::symbol("unquote"), parse_helper(iter)])
        },
        Some(&Token::QuasiQuote) => {
            iter.next();
            SExpr::List(vec![SExpr::symbol("quasiquote"), parse_helper(iter)])
        },
        Some(&Token::UnQuoteSplicing) => {
            iter.next();
            SExpr::List(vec![SExpr::symbol("unquote-splicing"), parse_helper(iter)])
        },
        Some(_) => {
            let y = iter.next().unwrap();
            SExpr::Atom(y)
        },
        None => panic!("Expected a token, found none."),
    }
}
