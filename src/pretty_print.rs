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
            match x {
                true  => "#t",
                false => "#f",
            }
        };
        
        match self {
            Token::LParen     => fmt.write_str("("),
            Token::RParen     => fmt.write_str(")"),
            Token::Symbol(x)  => fmt.write_str(x),
            Token::Integer(x) => fmt.write_str(&format!("{}", x)),
            Token::Float(x)   => fmt.write_str(&format!("{}", x)),
            Token::Boolean(x) => fmt.write_str(format_bool(x)),
            Token::Chr(x)     => fmt.write_str(&format!("#\\{}", x)),
            Token::Str(x)     => fmt.write_str(x),
        };

        Ok(())
    }
}

#[allow(unused_must_use)]
impl fmt::Display for SExpr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SExpr::Atom(x) => fmt.write_str(&format!("{}", x)),
            SExpr::List(xs) => {
                fmt.write_str(&format!("{}", Token::LParen));

                let mut sp = "";
                for x in xs {
                    fmt.write_str(sp);
                    fmt.write_str(&format!("{}", x));
                    sp = " ";
                }

                fmt.write_str(&format!("{}", Token::RParen))
            },
            SExpr::Procedure(x) => fmt.write_str(&format!("{}", x))
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
