use std::path::Path;
use std::fs::{File, remove_file, read_to_string};
use std::env;

use lexer::tokenize;
use parser::{parse, SExpr};
use evaluator::Args;

fn get_path_from_args(args: Args) -> String {
    let (file_name, _rest) = args.evaled()
        .own_one()
        .expect("Expected a filename as argument, found nothing.");

    file_name.into_str()
        .expect("Expected a string as argument, found something else.")
}

pub fn file_exists(args: Args) -> SExpr {
    SExpr::boolean(Path::new(&get_path_from_args(args)).exists())
}


pub fn delete_file(args: Args) -> SExpr {
    remove_file(get_path_from_args(args));
    SExpr::Unspecified
}

pub fn get_environment_variable(args: Args) -> SExpr {
    let var = env::var(get_path_from_args(args))
        .expect("Can't find env variable.");

    SExpr::str_owned(var)
}

pub fn get_environment_variables(_args: Args) -> SExpr {
    let vars = env::vars()
        .map(|(key, val)| SExpr::dottedlist(vec![SExpr::str_owned(key)], SExpr::str_owned(val)))
        .collect();

    SExpr::List(vars)
}

pub fn load(args: Args) -> SExpr {
    let env = args.env();
    let scm = read_to_string(get_path_from_args(args))
        .expect("Can not read file.");

    for sexpr in parse(tokenize(&scm)) {
        sexpr.eval(&env);
    }

    SExpr::Unspecified
}
