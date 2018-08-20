use std::ops::Index;

use lexer::Token;
use parser::SExpr;
use parser::SExprs;
use env::EnvRef;
use env::EnvRefT;
use serr::{SErr, SResult};

pub fn eval_mut_ref<F,T>(sexpr: &SExpr, env: &EnvRef, mut f: F) -> SResult<T>
where F: FnMut(&mut SExpr)->SResult<T> {
    match sexpr {
        SExpr::Atom(Token::Symbol(ref x)) => {
            env.with_mut_ref(x, |result| {
                match result {
                    SExpr::Lazy(_) => f(&mut result.eval(env)?),
                    _ => f(result)
                }
            })
        },
        x => f(&mut eval(x, env)?)
    }
}

pub fn eval_ref<F,T>(sexpr: &SExpr, env: &EnvRef, mut f: F) -> SResult<T>
where F: FnMut(&SExpr)->SResult<T> {
    match sexpr {
        SExpr::Atom(Token::Symbol(ref x)) => {
            env.with_ref(x, |result| {
                match result {
                    SExpr::Lazy(_) => f(&result.eval(env)?),
                    _ => f(result)
                }
            })
        },
        x => f(&eval(x, env)?)
    }
}

pub fn eval(sexpr: &SExpr, env: &EnvRef) -> SResult<SExpr> {
    match sexpr {
        SExpr::Atom(Token::Symbol(ref x)) => {
            let result = env.get(x)?;

            match result {
                SExpr::Lazy(_) => result.eval(env),
                _ => Ok(result)
            }
        },
        SExpr::Atom(x) => {
            Ok(SExpr::Atom(x.clone()))
        },
        SExpr::Procedure(x) => {
            Ok(SExpr::Procedure(x.clone()))
        },
        SExpr::Unspecified => {
            Ok(SExpr::Unspecified)
        },
        SExpr::Lazy(expr) => {
            expr.eval(&env)
        },
        SExpr::Vector(vec) => {
            Ok(SExpr::Vector(vec.clone()))
        },
        SExpr::Port(port) => {
            Ok(SExpr::Port(port.clone()))
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
                .ok_or_else(|| SErr::new_unexpected_form(sexpr))?;

            match op {
                SExpr::Atom(Token::Symbol(symbol)) => {
                    // Skip the op name
                    let args = xs[1..].to_args(&env);
                    call_procedure(symbol, args)
                },
                x => {
                    // Trying to use something other than a symbol as procedure
                    // Evaluate and see if it's a procedure.
                    let evaled = eval(x, env)?;
                    if let SExpr::Procedure(x) = evaled {
                        let args = xs[1..].to_args(&env);
                        x.apply(args)
                    } else {
                        bail!(NotAProcedure => x)
                    }
                }
            }
        }
    }
}

pub fn call_procedure(op: &str, args: Args) -> SResult<SExpr> {
    let procedure = args.env
        .get(op)?
        .clone();

    fn call(proc_expr: SExpr, args: Args) -> SResult<SExpr> {
        match proc_expr {
            SExpr::Procedure(proc) => proc.apply(args),
            SExpr::Lazy(p) => call(p.eval(&args.env)?, args),
            _ => bail!(NotAProcedure => proc_expr)
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

impl Index<usize> for Args {
    type Output = SExpr;

    fn index(&self, i: usize) -> &SExpr {
        self.get(i).unwrap()
    }
}

impl Args {
    pub fn new_with_extra(vec: SExprs, extra: Extra, env: &EnvRef) -> Args {
        Args { env: env.clone(), extra, vec }
    }

    pub fn new(vec: SExprs, env: &EnvRef) -> Args {
        Args { env: env.clone(), extra: Extra::Nothing, vec }
    }

    pub fn env(&self) -> EnvRef {
        self.env.clone()
    }

    pub fn into_all(self) -> SExprs {
        self.vec
    }

    pub fn into_split(self) -> SResult<(SExpr, SExprs)> {
        let mut iter = self.vec.into_iter();
        let head = iter.next();
        let tail = iter.collect();

        if head.is_some() {
            Ok((head.unwrap(), tail))
        } else {
            serr!(FoundNothing)
        }
    }

    pub fn get(&self, i: usize) -> Option<&SExpr> {
        self.vec.get(i)
    }

    pub fn all(&self) -> &SExprs {
        &self.vec
    }

    pub fn eval(&self) -> SResult<SExprs> {
        self.vec.iter()
            .map(|x| eval(&x, &self.env))
            .collect::<SResult<_>>()
    }

    pub fn evaled(self) -> SResult<Args> {
        Ok(Args::new_with_extra(self.eval()?, self.extra, &self.env))
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn own_one(self) -> SResult<SExpr> {
        let max = 1;
        if self.len() > max {
            bail!(WrongArgCount => max, self.len())
        }

        let mut iter = self.vec.into_iter();
        let x1 = iter.next().ok_or_else(|| SErr::WrongArgCount(max, 0))?;
        Ok(x1)
    }

    pub fn own_two(self) -> SResult<(SExpr, SExpr)> {
        let max = 2;
        if self.len() > max {
            bail!(WrongArgCount => max, self.len())
        }

        let mut iter = self.vec.into_iter();
        let x1 = iter.next().ok_or_else(|| SErr::WrongArgCount(max, 0))?;
        let x2 = iter.next().ok_or_else(|| SErr::WrongArgCount(max, 1))?;
        Ok((x1,x2))
    }

    pub fn own_three(self) -> SResult<(SExpr, SExpr, SExpr)> {
        let max = 3;
        if self.len() > max {
            bail!(WrongArgCount => max, self.len())
        }

        let mut iter = self.vec.into_iter();
        let x1 = iter.next().ok_or_else(|| SErr::WrongArgCount(max, 0))?;
        let x2 = iter.next().ok_or_else(|| SErr::WrongArgCount(max, 1))?;
        let x3 = iter.next().ok_or_else(|| SErr::WrongArgCount(max, 2))?;
        Ok((x1,x2,x3))
    }

    pub fn own_one_rest(self) -> SResult<(SExpr, SExprs)> {
        let mut iter = self.vec.into_iter();
        let x1 = iter.next().ok_or_else(|| SErr::WrongArgCount(1, 0))?;
        let rest = iter.collect();
        Ok((x1, rest))
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
