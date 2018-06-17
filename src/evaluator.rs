use lexer::Token;
use parser::SExpr;
use env::EnvRef;
use env::EnvRefT;

pub fn eval(sexpr: &SExpr, env: &EnvRef) -> SExpr {
    match sexpr {
        SExpr::Atom(Token::Symbol(ref x)) => {
            env.get(x)
                .expect(&format!("Unbound variable: {}", x))
        },
        SExpr::Atom(x) => {
            SExpr::Atom(x.clone())
        },
        SExpr::Procedure(x) => {
            SExpr::Procedure(x.clone())
        },
        SExpr::Unspecified => {
            SExpr::Unspecified
        },
        SExpr::Pair(pair) => {
            panic!("Evaluating dotted lists are not implemented yet!");
        },
        SExpr::List(xs) => {
            let op = xs.get(0)
                .expect("Expected an operator, found nothing.");

            match op {
                SExpr::Atom(Token::Symbol(symbol)) => {
                    // Skip the op name
                    let args = xs[1..].to_args(&env);
                    call_procedure(symbol, args)
                },
                x => {
                    // Trying to use something other than a symbol as procedure
                    // Evaluate and see if it's a procedure.
                    let evaled = eval(x, env);
                    if let SExpr::Procedure(x) = evaled {
                        let args = xs[1..].to_args(&env);
                        x.apply(args)
                    } else {
                        panic!("Wrong type to apply: {:#?}", x)
                    }
                }
            }
        }
    }
}

fn call_procedure(op: &str, args: Args) -> SExpr {
    let procedure = args.env
        .get(op)
        .expect(&format!("Unbound variable: {}", op));

    if let SExpr::Procedure(proc) = procedure {
        proc.apply(args)
    } else {
        panic!("Not a type to apply: {:#?}", procedure)
    }
}



#[derive(Debug)]
pub struct Args {
    pub env: EnvRef,
    vec: Vec<SExpr>
}

impl Args {
    pub fn new(vec: Vec<SExpr>, env: EnvRef) -> Args {
        Args {
            env: env,
            vec: vec
        }
    }
    
    pub fn get(&self, i: usize) -> Option<&SExpr> {
        self.vec.get(i)
    }
    
    pub fn all(&self) -> &Vec<SExpr> {
        &self.vec
    }

    // FIXME: iter -> into_iter?
    pub fn eval(&self) -> Vec<SExpr> {
        self.vec.iter()
            .map(|x| eval(&x, &self.env))
            .collect::<Vec<SExpr>>()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }
}


trait ToArgs {
    fn to_args(&self, env: &EnvRef) -> Args;
}


impl ToArgs for [SExpr] {
    fn to_args(&self, env: &EnvRef) -> Args {
        Args::new(self.to_vec(), env.clone_ref())
    }
}
