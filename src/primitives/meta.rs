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
