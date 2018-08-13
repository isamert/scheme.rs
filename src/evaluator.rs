use lexer::Token;
use parser::SExpr;
use parser::SExprs;
use env::EnvRef;
use env::EnvRefT;

pub fn eval_mut_ref<F,T>(sexpr: &SExpr, env: &EnvRef, mut f: F) -> T
where F: FnMut(&mut SExpr)->T {
    match sexpr {
        SExpr::Atom(Token::Symbol(ref x)) => {
            env.with_mut_ref(x, |var_ref| {
                let result = var_ref.expect(&format!("Unbound variable: {}", x));
                match result {
                    SExpr::Lazy(_) => f(&mut result.eval(env)),
                    _ => f(result)
                }
            })
        },
        x => f(&mut eval(x, env))
    }
}

pub fn eval_ref<F,T>(sexpr: &SExpr, env: &EnvRef, mut f: F) -> T
where F: FnMut(&SExpr)->T {
    match sexpr {
        SExpr::Atom(Token::Symbol(ref x)) => {
            env.with_ref(x, |var_ref| {
                let result = var_ref.expect(&format!("Unbound variable: {}", x));
                match result {
                    SExpr::Lazy(_) => f(&result.eval(env)),
                    _ => f(result)
                }
            })
        },
        x => f(&eval(x, env))
    }
}

pub fn eval(sexpr: &SExpr, env: &EnvRef) -> SExpr {
    match sexpr {
        SExpr::Atom(Token::Symbol(ref x)) => {
            let result = env.get(x)
                .expect(&format!("Unbound variable: {}", x));

            match result {
                SExpr::Lazy(_) => result.eval(env),
                _ => result
            }
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
        SExpr::Lazy(expr) => {
            expr.eval(&env)
        },
        SExpr::Vector(vec) => {
            SExpr::Vector(vec.clone())
        },
        SExpr::Port(port) => {
            SExpr::Port(port.clone())
        },
        list@SExpr::DottedList(_,_) => {
            fn flatten(list: &SExpr) -> SExprs {
                match list {
                    SExpr::DottedList(xs, sexpr) => {
                        let mut ys = xs.clone();
                        match &**sexpr {
                            SExpr::List(xs) => ys.append(&mut xs.clone()),
                            dl@SExpr::DottedList(_,_) => ys.append(&mut flatten(&dl)),
                            x => ys.push(x.clone())
                        };
                        ys
                    },
                    SExpr::List(xs) => {
                        xs.clone()
                    },
                    x => vec![x.clone()]
                }
            }
            SExpr::List(flatten(&list)).eval(env)
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
        .expect(&format!("Unbound variable: {}", op))
        .clone();

    fn call(proc_expr: SExpr, args: Args) -> SExpr {
        match proc_expr {
            SExpr::Procedure(proc) => proc.apply(args),
            SExpr::Lazy(p) => call(p.eval(&args.env), args),
            _ => panic!("Not a type to apply: {:#?}", proc_expr)
        }
    }

    call(procedure, args)
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
    vec: SExprs
}

impl Args {
    pub fn new_with_extra(vec: SExprs, extra: Extra, env: &EnvRef) -> Args {
        Args {
            env: env.clone(),
            extra: extra,
            vec: vec
        }
    }

    pub fn new(vec: SExprs, env: &EnvRef) -> Args {
        Args {
            env: env.clone(),
            extra: Extra::Nothing,
            vec: vec
        }
    }

    pub fn env(&self) -> EnvRef {
        self.env.clone()
    }

    pub fn into_all(self) -> SExprs {
        self.vec
    }

    pub fn into_split(self) -> Option<(SExpr, SExprs)> {
        let mut iter = self.vec.into_iter();
        let head = iter.next();
        let tail = iter.collect();

        if head.is_some() {
            Some((head.unwrap(), tail))
        } else {
            None
        }
    }

    pub fn get(&self, i: usize) -> Option<&SExpr> {
        self.vec.get(i)
    }

    pub fn all(&self) -> &SExprs {
        &self.vec
    }

    pub fn eval(&self) -> SExprs {
        self.vec.iter()
            .map(|x| eval(&x, &self.env))
            .collect::<SExprs>()
    }

    pub fn map<F>(mut self, mut f: F) -> Args
    where F: FnMut(SExpr) -> SExpr {
        self.vec = self.vec.into_iter()
            .map(|x| f(x))
            .collect::<SExprs>();

        self
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

// FIXME: does unneccesarry cloning when called, refactor with into_iter
pub trait ToArgs {
    fn to_args(&self, env: &EnvRef) -> Args;
}


impl ToArgs for [SExpr] {
    fn to_args(&self, env: &EnvRef) -> Args {
        Args::new(self.to_vec(), &env)
    }
}
