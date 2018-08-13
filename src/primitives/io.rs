use parser::SExpr;
use lexer::Token;
use evaluator::Args;
use ports::PortData;
use env::EnvRef;
use env::EnvRefT;


// input-port?
pub fn input_port__(args: Args) -> SExpr {
    SExpr::boolean(is_input_port(args))
}

// output-port?
pub fn output_port__(args: Args) -> SExpr {
    !SExpr::boolean(is_input_port(args))
}

pub fn open_input_file(args: Args) -> SExpr {
    SExpr::Port(PortData::new_textual_file_input(&get_path_from_args(args)))
}

pub fn open_output_file(args: Args) -> SExpr {
    SExpr::Port(PortData::new_textual_file_output(&get_path_from_args(args)))
}

pub fn read_line(args: Args) -> SExpr {
    let (_size, line) = args.get(0)
        .expect("Expected a port as argument, found nothing.")
        .eval_mut_ref(&args.env, |port_expr| {
            port_expr.as_port_mut()
                .expect("Expected a port as argument, found something else.")
                .read_line()
        });

    SExpr::str_(&line.trim_right_matches(|c| c == '\n'))
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
        let id = args.get(0).unwrap().as_symbol();
        if id.is_some() {
            let id_ = id.unwrap();
            args.env.remove(&id_)
                .expect(&format!("Unbind variable: {}", id_));
        } else {
            // This means port is created on the fly.
            // And it will get out of scope by itself.
        }
    }

    SExpr::Unspecified
}

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


fn is_input_port(args: Args) -> bool {
    args.get(0)
        .expect("Expected a port as argument, found nothing.")
        .eval_ref(&args.env, |port_expr| {
            let is_input = port_expr.as_port()
                .expect("Expected a port as argument, found something else.")
                .is_input();

            is_input
        })
}
