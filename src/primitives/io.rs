use parser::SExpr;
use evaluator::Args;
use ports::PortData;
use env::EnvRefT;


//
// Helpers
//
fn get_path_from_args(args: Args) -> String {
    if args.len() != 1 {
        panic!("Expected a file path, found nothing.")
    }

    let mut evaled_iter = args.eval().into_iter();
    evaled_iter.next()
        .unwrap()
        .into_str()
        .expect("Expected a string as argument, found something else.")
}

#[macro_export]
macro_rules! call_check_fn(
    ($args: ident, $fn: ident) => {
        {
            $args.get(0)
            .expect("Expected a port as argument, found nothing.")
            .eval_ref(&$args.env, |port_expr| {
                let is = port_expr.as_port()
                    .expect("Expected a port as argument, found something else.")
                    .$fn();

                SExpr::boolean(is)
            })
        }
    };
);

#[macro_export]
macro_rules! call_read_fn(
    ($args: ident, $fn: ident) => {
        {
            if $args.len() == 0 {
                let (_size, result) = PortData::current_input()
                    .$fn();

                result
            } else {
                let (_size, result) = $args.get(0)
                    .expect("Expected a port as argument, found nothing.")
                    .eval_mut_ref(&$args.env, |port_expr| {
                        port_expr.as_port_mut()
                            .expect("Expected a port as argument, found something else.")
                            .$fn()
                    });

                result
            }
        }
    };
);

#[macro_export]
macro_rules! call_write_fn(
    ($args: ident, $fn: ident, $thing: expr) => {
        {
            if $args.len() <= 1 {
                PortData::current_output()
                    .$fn(&$thing);
            } else if $args.len() > 1 {
                $args.get(1)
                    .expect("Expected a port as argument, found nothing.")
                    .eval_mut_ref(&$args.env, |port_expr| {
                        let port = port_expr.as_port_mut()
                            .expect("Expected a port as argument, found something else.");
                        port.$fn(&$thing);
                        SExpr::Unspecified
                    });
            }

            SExpr::Unspecified
        }
    };
);

//
// Functions
//
pub fn input_port__(args: Args) -> SExpr {
    call_check_fn!(args, is_input)
}

pub fn output_port__(args: Args) -> SExpr {
    call_check_fn!(args, is_output)
}

pub fn textual_port__(args: Args) -> SExpr {
    call_check_fn!(args, is_textual)
}

pub fn binary_port__(args: Args) -> SExpr {
    call_check_fn!(args, is_binary)
}

pub fn open_input_file(args: Args) -> SExpr {
    SExpr::Port(PortData::new_textual_file_input(&get_path_from_args(args)))
}

pub fn open_output_file(args: Args) -> SExpr {
    SExpr::Port(PortData::new_textual_file_output(&get_path_from_args(args)))
}

pub fn open_binary_input_file(args: Args) -> SExpr {
    SExpr::Port(PortData::new_binary_file_input(&get_path_from_args(args)))
}

pub fn open_binary_output_file(args: Args) -> SExpr {
    SExpr::Port(PortData::new_binary_file_output(&get_path_from_args(args)))
}

pub fn read_line(args: Args) -> SExpr {
    SExpr::str_(call_read_fn!(args, read_line).trim_right_matches(|c| c == '\n'))
}

pub fn read_char(args: Args) -> SExpr {
    SExpr::chr(call_read_fn!(args, read_char))
}

pub fn read_u8(args: Args) -> SExpr {
    SExpr::integer(i64::from(call_read_fn!(args, read_u8)))
}

pub fn read_all(args: Args) -> SExpr {
    args.get(0)
        .expect("Expected a port as argument, found nothing.")
        .eval_mut_ref(&args.env, |port_expr| {
            let port = port_expr.as_port_mut()
                .expect("Expected a port as argument, found something else.");

            if port.is_textual() && port.is_input() {
                let (_size, string) = port.read_all_str();
                SExpr::str_owned(string)
            } else if port.is_binary() && port.is_input() {
                let (_size, u8s) = port.read_all_u8();
                SExpr::List(u8s.into_iter().map(|u| SExpr::integer(i64::from(u))).collect())
            } else {
                panic!("The port is either closed or an input port, can't read.")
            }
        })
}

pub fn write(args: Args) -> SExpr {
    let string = args.get(0)
        .expect("Expected an argument, found nothing.")
        .eval(&args.env)
        .to_string();
    call_write_fn!(args, write_string, string)
}

pub fn write_string(args: Args) -> SExpr {
    // TODO: (write-string string port START)
    // TODO: (write-string string port START END)
    let string = args.get(0)
        .expect("Expected an argument, found nothing.")
        .eval(&args.env)
        .into_str()
        .expect("Expected a string as argument, found something else.");

    call_write_fn!(args, write_string, string)
}

pub fn newline(args: Args) -> SExpr {
    call_write_fn!(args, write_string, "\n")
}

pub fn display(args: Args) -> SExpr {
    let obj = args.get(0)
        .expect("Expected an argument, found nothing.")
        .eval(&args.env);

    let string = if obj.is_str() {
        obj.into_str().unwrap()
    } else if obj.is_chr() {
        obj.into_chr().unwrap().to_string()
    } else {
        obj.to_string()
    };

    call_write_fn!(args, write_string, string)
}

pub fn close_port(args: Args) -> SExpr {
    let mut remove = false;
    args.get(0)
        .expect("Expected a port as argument, found nothing.")
        .eval_ref(&args.env, |port_expr| match port_expr {
            SExpr::Port(_) => remove = true,
            x => panic!("Expected a port as argument, found this: {}", x)
        });

    // We can't directly remove it inside closure above, because env is
    // already borrowed, we can't borrow it twice.
    if remove {
        let id = args[0].as_symbol();
        if id.is_some() {
            let id_ = id.unwrap();
            let _port = args.env.set(id_.clone(), SExpr::Port(PortData::Closed))
                .expect(&format!("Unbind variable: {}", id_));
        } else {
            // This means port is created on the fly.
            // And it will get out of scope by itself.
        }
    }

    SExpr::Unspecified
}
