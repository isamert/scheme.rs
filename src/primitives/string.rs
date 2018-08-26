use parser::SExpr;
use evaluator::Args;
use serr::{SErr, SResult};


#[macro_export]
macro_rules! call_chr_fun(
    ($e: ident) => {
        |args| {
            let c = args.evaled()?.own_one()?;
            let result = c.into_chr()?.$e();
            Ok(result.into())
        }
    };
    ($e: ident !) => {
        |args| {
            let c = args.evaled()?.own_one()?;
            let result = c.into_chr()?.$e().next().unwrap();
            Ok(result.into())
        }
    };
);

#[macro_export]
macro_rules! call_str_fun(
    ($e: ident) => {
        |args| {
            let s = args.evaled()?.own_one()?;
            let result = s.into_str()?.$e();
            Ok(result.into())
        }
    };
);

pub fn string_copy(args: Args) -> SResult<SExpr> {
    let evaled = args.evaled()?;
    let string = if evaled.len() == 1 {
        evaled.own_one()?.into_str()?
    } else if evaled.len() == 2 {
        let (string_, start_) = evaled.own_two()?;
        let string = string_.into_str()?;
        let start = start_.into_int()? as usize;
        let len = string.len();
        string.get(start..)
            .ok_or_else(|| SErr::IndexOutOfBounds(len, start))?
            .to_string()
    } else if evaled.len() == 3 {
        let (string_, start_, end_) = evaled.own_three()?;
        let string = string_.into_str()?;
        let start = start_.into_int()? as usize;
        let end = end_.into_int()? as usize;
        let len = string.len();
        string.get(start..end)
            .ok_or_else(|| SErr::IndexOutOfBounds(len, end))?
            .to_string()
    } else {
        bail!(WrongArgCount => 3 as usize, evaled.len())
    };

    Ok(sstr!(string))
}

pub fn string_append(args: Args) -> SResult<SExpr> {
    let result = args.evaled()?
        .into_iter()
        .map(|x| x.into_str())
        .collect::<SResult<Vec<String>>>()?
        .join("");

    Ok(sstr!(result))
}
