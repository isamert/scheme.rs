use std::ops::Index;

use lexer::Token;
use parser::SExpr;
use parser::SExprs;
use env::EnvRef;
use env::EnvRefT;
use procedure::{ProcedureData, PrimitiveData, CompoundData};
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

pub fn eval(sexpr_: &SExpr, env_: &EnvRef) -> SResult<SExpr> {
    let mut sexpr = sexpr_.clone();
    let mut env = env_.clone_ref();
    loop {
        match sexpr {
            SExpr::Atom(Token::Symbol(x)) => {
                let result = env.get(&x)?;

                match result {
                    SExpr::Lazy(_) => sexpr = result,
                    _ => return Ok(result)
                };
            },
            x@SExpr::Atom(_) | x@SExpr::Procedure(_) | x@SExpr::Vector(_)
                | x@SExpr::Port(_) | x@SExpr::Unspecified => {
                return Ok(x)
            },
            SExpr::Lazy(expr) => {
                sexpr = *expr;
            },
            list@SExpr::DottedList(_,_) => {
                fn flatten(list: SExpr) -> SExprs {
                    match list {
                        SExpr::DottedList(xs, sexpr) => {
                            let mut ys = xs;
                            match *sexpr {
                                SExpr::List(mut xs) => ys.append(&mut xs),
                                dl@SExpr::DottedList(_,_) => ys.append(&mut flatten(dl)),
                                x => ys.push(x)
                            };
                            ys
                        },
                        SExpr::List(xs) => xs,
                        x => vec![x]
                    }
                }

                sexpr = SExpr::List(flatten(list));
            },
            SExpr::List(xs) => {
                let mut iter = xs.into_iter();
                let op = iter.next()
                    .ok_or_else(|| SErr::new_unexpected_form(&SExpr::List(vec![])))?;
                let args = Args::new(iter.collect(), &env);

                match op {
                    // Need to handle control structres like if and begin
                    // here to be able to use same stack for tail recursive
                    // functions.
                    // Other control structres should be written in forms of
                    // if or begin (and I hope that's all for basic TCO)
                    SExpr::Atom(Token::Symbol(ref sym)) if sym == "if" => {
                        let mut arg_iter = args.into_all().into_iter();
                        let test = arg_iter.next()
                            .ok_or_else(|| SErr::WrongArgCount(2, 0))?;
                        let consequent = arg_iter.next()
                            .ok_or_else(|| SErr::WrongArgCount(2, 1))?;
                        let alterne = arg_iter.next()
                            .unwrap_or(SExpr::Unspecified);

                        if test.eval(&env)?.to_bool() {
                            sexpr = consequent;
                        } else {
                            sexpr = alterne;
                        }
                    },
                    SExpr::Atom(Token::Symbol(ref sym)) if sym == "begin" => {
                        sexpr = args.eval()?
                            .into_iter()
                            .last()
                            .unwrap_or_else(|| SExpr::Unspecified);
                    },
                    SExpr::Atom(Token::Symbol(symbol)) => {
                        let procedure = args.env
                            .get(&symbol)?
                            .clone();

                        match procedure {
                            SExpr::Procedure(proc) => match proc {
                                ProcedureData::Primitive(x) => return x.apply(args),
                                ProcedureData::Compound(x) => {
                                    env = x.build_env(args)?;
                                    sexpr = *x.body;
                                }
                            },
                            // FIXME: SExpr::Lazy(p) => call(p.eval(&args.env)?, args),
                            _ => bail!(NotAProcedure => procedure)
                        };
                    },
                    x => {
                        // Trying to use something other than a symbol as procedure
                        // Evaluate and see if it's a procedure.
                        let evaled = eval(&x, &env)?;
                        if let SExpr::Procedure(procedure) = evaled {
                            match procedure {
                                ProcedureData::Primitive(x) => return x.apply(args),
                                ProcedureData::Compound(x) => {
                                    env = x.build_env(args)?;
                                    sexpr = *x.body;
                                }
                            };
                        } else {
                            bail!(NotAProcedure => x)
                        }
                    }
                }
            }
        };
    };
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
