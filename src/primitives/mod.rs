pub mod lang;
pub mod equivalence;
#[macro_use]
pub mod numeric;
pub mod ordering;
pub mod conditionals;
pub mod list;
#[macro_use]
pub mod string;
pub mod io;
pub mod system;
pub mod prelude;
pub mod meta;

use primitives::prelude::PRELUDE;
use env::{EnvRef, EnvValues};
use lexer::tokenize;
use parser::parse;
use serr::SResult;

pub fn load_prelude(env: &EnvRef) -> SResult<()> {
    for sexpr in parse(tokenize(&mut PRELUDE.to_string().chars().into_iter().peekable()))? {
        sexpr.eval(&env)?;
    }
    Ok(())
}

pub fn env() -> EnvValues {
    environment! {
        "typeof"  => meta::type_of,

        "define"      => lang::define,
        "set!"        => lang::set,
        "lambda"      => lang::lambda,
        "apply"       => lang::apply,
        "let"         => lang::let_,
        "let*"        => lang::let_star,
        "letrec"      => lang::let_rec,
        "quote"       => lang::quote,
        "quasiquote"  => lang::quasiquote,
        "unquote"     => lang::unquote,

        "eqv?"   => equivalence::eqv_qm,
        "eq?"    => equivalence::eq_qm,
        "equal?" => equivalence::equal_qm,

        "+"  => |args| numeric::calc('+', args),
        "-"  => |args| numeric::calc('-', args),
        "*"  => |args| numeric::calc('*', args),
        "/"  => |args| numeric::calc('/', args),
        "remainder"   => numeric::remainder,
        "modulo"      => numeric::modulo,
        "numerator"   => numeric::numerator,
        "denominator" => numeric::denominator,
        "sqrt"        => call_float_fun!(sqrt),
        "expt"        => call_float_fun!(sqrt),
        "ceiling"     => call_float_fun!(ceil),
        "floor"       => call_float_fun!(floor),
        "truncate"    => call_float_fun!(trunc),
        "round"       => call_float_fun!(round),
        "exp"         => call_float_fun!(exp),
        "log"         => call_float_fun!(ln, log),
        "sin"         => call_float_fun!(sin),
        "cos"         => call_float_fun!(cos),
        "tan"         => call_float_fun!(tan),
        "asin"        => call_float_fun!(asin),
        "acos"        => call_float_fun!(acos),
        "atan"        => call_float_fun!(atan, atan2),
        "number->string" => numeric::number_string,
        "string->number" => numeric::string_number,

        "<"  => ordering::lt,
        ">"  => ordering::gt,
        "<=" => ordering::lte,
        ">=" => ordering::gte,
        "="  => ordering::eq,

        "cond" => conditionals::cond,
        "case" => conditionals::case,
        "and"  => conditionals::and,
        "or"   => conditionals::or,

        "cons"   => list::cons,
        "car"    => list::car,
        "cdr"    => list::cdr,
        "append" => list::append,

        "string-upcase"      => call_str_fun!(to_uppercase),
        "string-downcase"    => call_str_fun!(to_lowercase),
        "string-length"      => call_str_fun!(len),
        "char-upcase"        => call_chr_fun!(to_uppercase !),
        "char-downcase"      => call_chr_fun!(to_lowercase !),
        "char-upper-case?"   => call_chr_fun!(is_uppercase),
        "char-lower-case?"   => call_chr_fun!(is_lowercase),
        "char-alphabetic?"   => call_chr_fun!(is_alphabetic),
        "char-numeric?"      => call_chr_fun!(is_numeric),
        "char-alphanumeric?" => call_chr_fun!(is_alphanumeric),
        "char-whitespace?"   => call_chr_fun!(is_whitespace),
        "string-copy"        => string::string_copy,

        "load"         => system::load,
        "file-exists?" => system::file_exists_qm,
        "delete-file"  => system::delete_file,
        "system*"      => system::system_star,
        "get-environment-variable"  => system::get_environment_variable,
        "get-environment-variables" => system::get_environment_variables,

        "open-binary-input-file"  => io::open_binary_input_file,
        "open-binary-output-file" => io::open_binary_output_file,
        "open-input-file"  => io::open_input_file,
        "open-output-file" => io::open_output_file,
        "read"             => io::read,
        "read-u8"          => io::read_u8,
        "read-line"        => io::read_line,
        "read-char"        => io::read_char,
        "read-all"         => io::read_all,
        "write"            => io::write,
        "write-string"     => io::write_string,
        "display"          => io::display,
        "newline"          => io::newline,
        "close-port"       => io::close_port
    }
}
