use std::fmt;

use lexer::Token;
use parser::SExpr;
use procedure::ProcedureData;
use procedure::CompoundData;
use procedure::PrimitiveData;

#[allow(unused_must_use)]
impl fmt::Display for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let format_bool = |&x| {
            if x { "#t" } else { "#f" }
        };
        let s = match self {
            Token::LParen          => "(".to_string(),
            Token::RParen          => ")".to_string(),
            Token::Dot             => ".".to_string(),
            Token::Ellipsis        => "...".to_string(),
            Token::Quote           => "'".to_string(),
            Token::UnQuote         => ",".to_string(),
            Token::QuasiQuote      => "`".to_string(),
            Token::UnQuoteSplicing => ",@".to_string(),
            Token::Symbol(x)  => x.to_string(),
            Token::Integer(x) => format!("{}", x),
            Token::Float(x)   => format!("{}", x),
            Token::Fraction(x) => format!("{}/{}", x.n, x.d),
            Token::Boolean(x) => format_bool(x).to_string(),
            Token::Chr(x)     => format!("#\\{}", x),
            Token::Str(x)     => format!("\"{}\"", x),
        };

        fmt.write_str(&s);

        Ok(())
    }
}

#[allow(unused_must_use)]
impl fmt::Display for SExpr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SExpr::Atom(x) => fmt.write_str(&format!("{}", x)),
            SExpr::Procedure(x) => fmt.write_str(&format!("{}", x)),
            SExpr::Unspecified => fmt.write_str("<unspecified>"),
            SExpr::Port(_port) => fmt.write_str("#<a port>"),
            SExpr::DottedList(xs, sexpr) => fmt.write_str(&format!("({} . {})", str_list(xs), sexpr)),
            SExpr::List(xs) => fmt.write_str(&format!("({})", str_list(xs))),
        };
        Ok(())
    }
}

#[allow(unused_must_use)]
impl fmt::Display for ProcedureData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProcedureData::Compound(x)  => fmt.write_str(&format!("{}", x)),
            ProcedureData::Primitive(x) => fmt.write_str(&format!("{}", x)),
        };
        Ok(())
    }
}


#[allow(unused_must_use)]
impl fmt::Display for CompoundData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&format!("#<compound-procedure {:?}>", self as *const _));
        Ok(())
    }
}

#[allow(unused_must_use)]
impl fmt::Display for PrimitiveData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&format!("#<primitive-procedure {:?}>", self as *const _));
        Ok(())
    }
}

#[allow(unused_must_use)]
fn str_list(xs: &[SExpr]) -> String {

    let mut lstr = String::new();
    let mut sp = "";
    for x in xs {
        lstr.push_str(sp);
        lstr.push_str(&format!("{}", x));
        sp = " ";
    }

    lstr
}
