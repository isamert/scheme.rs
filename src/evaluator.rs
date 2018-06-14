use lexer::Token;
use parser::SExpr;
use env::EnvRef;
use env::EnvRefT;

pub fn eval(sexpr: &SExpr, env: EnvRef) -> SExpr {
    match sexpr {
        SExpr::Atom(Token::Symbol(ref x)) => {
            env.get(x)
        },
        SExpr::Atom(x) => {
            SExpr::Atom(x.clone())
        },
        SExpr::Procedure(_) => {
            panic!("YOU FUCKED UP")
        },
        SExpr::List(xs) => {
            let op = xs.get(0)
                .expect("Expected an operator, found nothing.");

            match op {
                SExpr::Atom(Token::Symbol(symbol)) => {
                    // Skip the op name
                    let args = xs[1..].to_vec();
                    call_function(symbol, args, env.clone_ref())
                },
                x => {
                    // Trying to use something other than a symbol as function
                    panic!("Wrong type to apply: {:#?}", x);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Args {
    env: EnvRef,
    vec: Vec<SExpr>
}

// FIXME: add new()
impl Args {
    pub fn get(&self, i: usize) -> Option<&SExpr> {
        self.vec.get(i)
    }
    
    pub fn all(&self) -> &Vec<SExpr> {
        &self.vec
    }

    // FIXME: iter -> into_iter?
    pub fn eval(&self) -> Vec<SExpr> {
        self.vec.iter()
            .map(|x| eval(&x, self.env.clone_ref()))
            .collect::<Vec<SExpr>>()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn env(&self) -> EnvRef {
        self.env.clone_ref()
    }
}

fn call_function(op: &str, args_vec: Vec<SExpr>, env: EnvRef) -> SExpr {
    let args = Args {
        env: env.clone_ref(),
        vec: args_vec
    };
    let procedure = env.get(op);
    if let SExpr::Procedure(proc) = procedure {
        proc.apply(args)
    } else {
        panic!("Not a type to apply: {:#?}", procedure)
    }
}



