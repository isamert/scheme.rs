use lexer::TokenIterator;
use parser::{SExpr, parse_single};
use evaluator::Args;
use port::{PortData, current_input_port, current_output_port};
use env::EnvRefT;
use serr::{SErr, SResult};

//
// Helpers
//
fn get_path_from_args(args: Args) -> SResult<String> {
    if args.len() != 1 {
        bail!(WrongArgCount => 1 as usize, 0 as usize)
    }

    let mut evaled_iter = args.eval()?.into_iter();
    evaled_iter.next()
        .unwrap()
        .into_str()
}

macro_rules! call_check_fn(
    ($args: ident, $fn: ident) => {
        {
            $args.get(0)
            .ok_or_else(|| SErr::WrongArgCount(1, 0))?
            .eval_ref(&$args.env, |port_expr| {
                let is = port_expr.as_port()?
                    .$fn();

                Ok(sbool!(is))
            })
        }
    };
);

macro_rules! call_read_fn(
    ($args: ident, $fn: ident) => {
        {
            if $args.len() == 0 {
                let (_size, result) = current_input_port().$fn()?;
                Ok(result)
            } else {
                let (_size, result) = $args.get(0)
                    .ok_or_else(|| SErr::WrongArgCount(1, 0))?
                    .eval_mut_ref(&$args.env, |port_expr| {
                        port_expr.as_port_mut()?
                            .$fn()
                    })?;

                Ok(result)
            }
        }
    };
);

macro_rules! call_write_fn(
    ($args: ident, $fn: ident, $thing: expr) => {
        {
            if $args.len() <= 1 {
                current_output_port().$fn(&$thing)?;
            } else if $args.len() > 1 {
                $args.get(1)
                    .ok_or_else(|| SErr::WrongArgCount(1, 0))?
                    .eval_mut_ref(&$args.env, |port_expr| {
                        let port = port_expr.as_port_mut()?;
                        port.$fn(&$thing)?;
                        Ok(SExpr::Unspecified)
                    })?;
            }

            Ok(SExpr::Unspecified)
        }
    };
);

//
// Functions
//
pub fn input_port_qm(args: Args) -> SResult<SExpr> {
    call_check_fn!(args, is_input)
}

pub fn output_port_qm(args: Args) -> SResult<SExpr> {
    call_check_fn!(args, is_output)
}

pub fn textual_port_qm(args: Args) -> SResult<SExpr> {
    call_check_fn!(args, is_textual)
}

pub fn binary_port_qm(args: Args) -> SResult<SExpr> {
    call_check_fn!(args, is_binary)
}

pub fn open_input_file(args: Args) -> SResult<SExpr> {
    Ok(SExpr::Port(PortData::new_textual_file_input(&get_path_from_args(args)?)?))
}

pub fn open_output_file(args: Args) -> SResult<SExpr> {
    Ok(SExpr::Port(PortData::new_textual_file_output(&get_path_from_args(args)?)?))
}

pub fn open_binary_input_file(args: Args) -> SResult<SExpr> {
    Ok(SExpr::Port(PortData::new_binary_file_input(&get_path_from_args(args)?)?))
}

pub fn open_binary_output_file(args: Args) -> SResult<SExpr> {
    Ok(SExpr::Port(PortData::new_binary_file_output(&get_path_from_args(args)?)?))
}

pub fn read(args: Args) -> SResult<SExpr> {
    // I just couldn't define this closure as a simple variable
    macro_rules! parse_chars(() => {
        |chars| {
            let mut iter = TokenIterator::new(chars).peekable();
            Ok(parse_single(&mut iter)?)
        }
    };);

    if args.len() == 0 {
        current_input_port().with_chars(parse_chars!())
    } else {
        args.get(0)
            .ok_or_else(|| SErr::WrongArgCount(1, 0))?
            .eval_mut_ref(&args.env, |port_expr| {
                port_expr.as_port_mut()?
                    .with_chars(parse_chars!())
            })
    }
}

pub fn read_line(args: Args) -> SResult<SExpr> {
    // I couldn't understand why it can't infer the type of x.
    let x: SResult<String> = call_read_fn!(args, read_line);
    Ok(sstr!(x?.trim_right_matches(|c| c == '\n')))
}

pub fn read_char(args: Args) -> SResult<SExpr> {
    let x: SResult<char> = call_read_fn!(args, read_char);
    Ok(schr!(x?))
}

pub fn read_u8(args: Args) -> SResult<SExpr> {
    let x: SResult<u8> = call_read_fn!(args, read_u8);
    Ok(sint!(i64::from(x?)))
}

pub fn read_all(args: Args) -> SResult<SExpr> {
    args.get(0)
        .ok_or_else(|| SErr::WrongArgCount(1, 0))?
        .eval_mut_ref(&args.env, |port_expr| {
            let port = port_expr.as_port_mut()?;

            if port.is_textual() && port.is_input() {
                let (_size, string) = port.read_all_str()?;
                Ok(sstr!(string))
            } else if port.is_binary() && port.is_input() {
                let (_size, u8s) = port.read_all_u8()?;
                Ok(SExpr::List(u8s.into_iter().map(|u| sint!(i64::from(u))).collect()))
            } else {
                bail!(TypeMismatch => "a textual or binary input port", SExpr::Port(port.clone()))
            }
        })
}

pub fn write(args: Args) -> SResult<SExpr> {
    let string = args.get(0)
        .ok_or_else(|| SErr::WrongArgCount(1, 0))?
        .eval(&args.env)?
        .to_string();
    call_write_fn!(args, write_string, string)
}

pub fn write_string(args: Args) -> SResult<SExpr> {
    // TODO: (write-string string port START)
    // TODO: (write-string string port START END)
    let string = args.get(0)
        .ok_or_else(|| SErr::WrongArgCount(1, 0))?
        .eval(&args.env)?
        .into_str()?;

    call_write_fn!(args, write_string, string)
}

pub fn newline(args: Args) -> SResult<SExpr> {
    call_write_fn!(args, write_string, "\n")
}

pub fn display(args: Args) -> SResult<SExpr> {
    let obj = args.get(0)
        .ok_or_else(|| SErr::WrongArgCount(1, 0))?
        .eval(&args.env)?;

    let string = if obj.is_str() {
        obj.into_str().unwrap()
    } else if obj.is_chr() {
        obj.into_chr().unwrap().to_string()
    } else {
        obj.to_string()
    };

    call_write_fn!(args, write_string, string)
}

pub fn close_port(args: Args) -> SResult<SExpr> {
    let mut remove = false;
    args.get(0)
        .ok_or_else(|| SErr::WrongArgCount(1, 0))?
        .eval_ref(&args.env, |port_expr| match port_expr {
            SExpr::Port(_) => {
                remove = true;
                Ok(())
            },
            x => bail!(TypeMismatch => "port", x)
        })?;

    // We can't directly remove it inside closure above, because env is
    // already borrowed, we can't borrow it twice mutably.
    if remove {
        let id = args[0].as_symbol();
        if id.is_ok() {
            let id_ = id.unwrap().clone();
            args.env.set(id_, SExpr::Port(PortData::Closed))?;
        } else {
            // This means port is created on the fly.
            // And it will get out of scope by itself.
        }
    }

    Ok(SExpr::Unspecified)
}
