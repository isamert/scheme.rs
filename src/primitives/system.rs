use std::path::Path;
use std::fs::{remove_file, read_to_string};
use std::env;
use std::process::Command;

use lexer::tokenize;
use parser::{parse, SExpr};
use evaluator::Args;
use serr::SResult;

fn get_path_from_args(args: Args) -> SResult<String> {
    args.evaled()?
        .own_one()?
        .into_str()
}

pub fn file_exists_qm(args: Args) -> SResult<SExpr> {
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
        let result = sexpr.eval(&env)?;
        if !result.is_unspecified() {
            println!("{}", result);
        }
    }

    Ok(SExpr::Unspecified)
}

// system*
pub fn system_star(args: Args) -> SResult<SExpr> {
    let (cmd_expr, arg_expr) = args.evaled()?.own_one_rest()?;
    let cmd = cmd_expr.into_str()?;
    let argus = arg_expr.into_iter()
        .map(|x| x.into_str())
        .collect::<SResult<Vec<_>>>()?;

    let status = Command::new(cmd)
        .args(argus)
        .status()?
        .code()
        .unwrap_or(1);

    Ok(SExpr::integer(status as i64))
}
