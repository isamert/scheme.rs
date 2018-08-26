use std::char;

use lexer::Token::*;
use parser::SExpr::*;
use parser::SExpr;
use serr::{SErr,SResult};
use evaluator::Args;
use port::PortData::*;

pub fn type_of(args: Args) -> SResult<SExpr> {
    let item = args.evaled()?.own_one()?;

    Ok(match item {
        Atom(Symbol(_)) => ssymbol!("symbol"),
        Atom(Integer(_)) => ssymbol!("integer"),
        Atom(Fraction(_)) => ssymbol!("fraction"),
        Atom(Float(_)) => ssymbol!("float"),
        Atom(Boolean(_)) => ssymbol!("boolean"),
        Atom(Chr(_)) => ssymbol!("chr"),
        Atom(Str(_)) => ssymbol!("str"),
        Atom(_) => ssymbol!("atom"),
        List(_) => ssymbol!("list"),
        DottedList(_,_) => ssymbol!("list-dotted"),
        Procedure(_) => ssymbol!("procedure"),
        Port(TextualFileInput(_,_)) => ssymbol!("port-textual-in"),
        Port(TextualFileOutput(_,_)) => ssymbol!("port-textual-out"),
        Port(BinaryFileInput(_,_)) => ssymbol!("port-binary-in"),
        Port(BinaryFileOutput(_,_)) => ssymbol!("port-binary-out"),
        Port(StdInput(_)) => ssymbol!("port-std-in"),
        Port(StdOutput(_)) => ssymbol!("port-std-out"),
        Port(Closed) => ssymbol!("port-closed"),
        _ => bail!(Generic => "Is that a thing?")
    })
}

pub fn convert_type(args: Args) -> SResult<SExpr> {
    let (typ, arg) = args.evaled()?.own_two()?;

    Ok(match typ {
        Atom(Symbol(ref t)) if t == "symbol" => match arg {
            x@Atom(Symbol(_)) => x,
            Atom(Str(x)) => ssymbol!(x),
            Atom(Chr(x)) => ssymbol!(x.to_string()),
            x => bail!(Cast => "symbol", x)
        },
        Atom(Symbol(ref t)) if t == "chr" => match arg {
            x@Atom(Chr(_)) => x,
            Atom(Integer(x)) => {
                let result = char::from_u32(x as u32)
                    .ok_or_else(|| SErr::Cast("chr".to_string(), sint!(x)))?;

                schr!(result)
            },
            Atom(Str(x)) => {
                let result = x.chars()
                    .next()
                    .ok_or_else(|| SErr::new_generic("Can't convert empty string to char."))?;

                schr!(result)
            },
            x => bail!(Cast => "chr", x)
        },
        Atom(Symbol(ref t)) if t == "integer" => match arg {
            x@Atom(Integer(_)) => x,
            Atom(Chr(x)) => sint!(x as i64),
            x => bail!(Cast => "chr", x)
        },
        Atom(Symbol(ref t)) if t == "str" => match arg {
            x@Atom(Str(_)) => x,
            Atom(Symbol(x)) => sstr!(x),
            Atom(Chr(x)) => sstr!(x.to_string()),
            List(xs) => {
                let result = xs.into_iter()
                    .map(|x| x.into_chr())
                    .collect::<SResult<String>>()?;

                sstr!(result)
            },
            DottedList(xs, y) => {
                let mut result = xs.into_iter()
                    .map(|x| x.into_chr())
                    .collect::<SResult<String>>()?;
                result.push(y.into_chr()?);

                sstr!(result)
            },
            x => bail!(Cast => "str", x)
        },
        Atom(Symbol(ref t)) if t == "list" => match arg {
            Atom(Str(x)) => {
                let result = x.chars()
                    .map(|c| schr!(c))
                    .collect();

                List(result)
            },
            x@List(_) => x,
            DottedList(mut xs, y) => {
                xs.push(*y);
                List(xs)
            },
            x => bail!(Cast => "list", x)
        },
        x => bail!(TypeMismatch => x.into_symbol()?, arg)
    })
}
