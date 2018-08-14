#[macro_use]
mod util;

mod env;
mod lexer;
mod parser;
mod ports;
mod procedure;
mod evaluator;
mod primitives;
mod pretty_print;
mod repl;

use env::Env;

fn main() {
    let env = Env::with_values(Env::null(), primitives::env());
    repl::run(env.into_ref());
}

