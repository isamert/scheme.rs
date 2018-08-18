use std::path::Path;
use std::fs::{remove_file, read_to_string};
use std::env;

use lexer::tokenize;
use parser::{parse, SExpr};
use evaluator::Args;
use serr::SResult;

fn get_path_from_args(args: Args) -> SResult<String> {
    let (file_name, _rest) = args.evaled()?
        .own_one()?;

    file_name.into_str()
}

pub fn file_exists(args: Args) -> SResult<SExpr> {
    Ok(SExpr::boolean(Path::new(&get_path_from_args(args)?).exists()))
}


pub fn delete_file(args: Args) -> SResult<SExpr> {
    remove_file(get_path_from_args(args)?)?;
    Ok(SExpr::Unspecified)
}

pub fn get_environment_variable(args: Args) -> SResult<SExpr> {
    let var = env::var(get_path_from_args(args)?)?;
    Ok(SExpr::str_owned(var))
}

pub fn get_environment_variables(_args: Args) -> SResult<SExpr> {
    let vars = env::vars()
        .map(|(key, val)| SExpr::dottedlist(vec![SExpr::str_owned(key)], SExpr::str_owned(val)))
        .collect();

    Ok(SExpr::List(vars))
}

pub fn load(args: Args) -> SResult<SExpr> {
    let env = args.env();
    let scm = read_to_string(get_path_from_args(args)?)?;

    for sexpr in parse(tokenize(&scm))? {
        sexpr.eval(&env)?;
    }

    Ok(SExpr::Unspecified)
}
