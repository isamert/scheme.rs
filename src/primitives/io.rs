use parser::SExpr;
use lexer::Token;
use evaluator::Args;
use ports::PortData;

pub fn open_input_file(args: Args) -> SExpr {
    let mut evaled_iter = args.eval().into_iter();
    let path = evaled_iter.next()
        .expect("Expected a file path, found nothing.")
        .into_str()
        .expect("Expected a string as argument, found something else.");

    SExpr::Port(PortData::new_file_input(&path))
}

pub fn read_line(args: Args) -> SExpr {
    let mut evaled_iter = args.eval().into_iter();
    let port = evaled_iter.next()
        .expect("Expected a port as argument, found nothing.")
        .into_port()
        .expect("Expected a port as argument, found something else.");
    let (_size, line) = port.read_line();

    SExpr::str_(&line)
}
