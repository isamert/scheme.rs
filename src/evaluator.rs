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
        SExpr::Pair(ref pair) => {
            // FIXME: not a correct implementation
            let head = pair.0.clone();
            let tail = pair.1.clone();

            println!("head: {} || tail {}", head, tail);

            // If the tail is also a list, then flatten the pair and eval it
            if let SExpr::List(mut xs) = tail {
                let flatten = {
                    xs.insert(0, head);
                    SExpr::List(xs)
                };
                flatten.eval(&env)
            } else {
                panic!("Can not evaluate.");
            }
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

pub fn call_procedure(op: &str, args: Args) -> SExpr {
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
pub enum Extra {
    QQLevel(usize),
    Nothing
}

#[derive(Debug)]
pub struct Args {
    pub env: EnvRef,
    pub extra: Extra,
    vec: Vec<SExpr>
}

impl Args {
    pub fn new(vec: Vec<SExpr>, extra: Extra, env: &EnvRef) -> Args {
        Args {
            env: env.clone(),
            extra: extra,
            vec: vec
        }
    }

    pub fn head(&self) -> &SExpr {
        self.vec
            .first()
            .expect("Expected an argument, found nothing.")
    }

    pub fn tail(&self) -> &SExpr {
        self.vec
            .first()
            .expect("Expected an argument, found nothing.")
    }

    pub fn with_head(mut self) -> Args {
        let head = self.vec.pop()
            .expect("Expected an argument, found nothing");
        self.vec.clear();
        self.vec.push(head);
        self
    }

    pub fn with_tail(mut self) -> Args {
        self.vec.pop();
        self
    }

    pub fn into_all(mut self) -> Vec<SExpr> {
        self.vec
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


pub trait ToArgs {
    fn to_args(&self, env: &EnvRef) -> Args;
}


impl ToArgs for [SExpr] {
    fn to_args(&self, env: &EnvRef) -> Args {
        Args::new(self.to_vec(), Extra::Nothing, &env)
    }
}
