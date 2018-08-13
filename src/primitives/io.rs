use parser::SExpr;
use lexer::Token;
use evaluator::Args;
use ports::PortData;
use env::EnvRef;
use env::EnvRefT;

pub fn open_input_file(args: Args) -> SExpr {
    let mut evaled_iter = args.eval().into_iter();
    let path = evaled_iter.next()
        .expect("Expected a file path, found nothing.")
        .into_str()
        .expect("Expected a string as argument, found something else.");

    SExpr::Port(PortData::new_textual_file_input(&path))
}

pub fn read_line(args: Args) -> SExpr {
    let (_size, line) = args.get(0)
        .expect("Expected a port as argument, found nothing.")
        .eval_mut_ref(&args.env, |port_expr| {
            port_expr.as_port()
                .expect("Expected a port as argument, found something else.")
                .read_line()
        });

    SExpr::str_(&line.trim_right_matches(|c| c == '\n'))
}
