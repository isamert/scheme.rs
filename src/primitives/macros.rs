use lexer::Token;
use lexer::tokenize;
use parser::{SExpr, parse, SExprs};
use evaluator::Args;
use serr::{SErr, SResult};

pub fn syntax_rules(args: Args) -> SResult<SExpr> {
    let (literals_, rules_) = args.own_one_rest()?;
    let literals = literals_.into_list()?;

    let rules = rules_.into_iter().map(|rule| {
        let mut rule_iter = rule.into_list()?.into_iter();
        let pattern = rule_iter.next().ok_or_else(|| SErr::WrongArgCount(2, 0))?;
        let template = rule_iter.next().ok_or_else(|| SErr::WrongArgCount(2, 1))?;

        Ok((pattern, template))
    }).collect::<SResult<Vec<(SExpr, SExpr)>>>()?;

    //let candidate = parse(tokenize("[xxx 1 (1 2) (3 4) (5 6) (7 8)]"))?.get(0).unwrap().clone();
    let candidate = parse(tokenize("(let ((x 1) (y 2) (z 3)) (body1) (body2))"))?.get(0).unwrap().clone();
    println!("CANDIDATE: {}", candidate);
    for (pattern, template_) in &rules {
    println!("pattern: {:}", pattern);
        if let Some(bindings) = pattern_matches(pattern.clone(), candidate.clone()) {
            println!("{:?}", bindings);
        } else {
            println!("NOTMATCHED");
        }
    };

    Ok(SExpr::Unspecified)
}

#[derive(Debug)]
pub struct BindingBuilder {
    last_pattern_index: Option<usize>,
    vec: Vec<Binding>,
}

impl BindingBuilder {
    fn new() -> BindingBuilder {
        BindingBuilder {
            last_pattern_index: None,
            vec: vec![]
        }
    }

    fn put(&mut self, key: SExpr, val: SExpr) {
        for mut p in &mut self.vec {
            if p.key == key {
                p.add_val(val);
                return;
            }
        }

        self.vec.push(Binding::new_with_value(key, val));
    }
}

#[derive(Debug)]
pub struct Binding {
    key: SExpr,
    vec: Vec<(usize, SExpr)>,
    level: usize
}

impl Binding {
    fn new(key: SExpr) -> Binding {
        Binding {
            key,
            vec: vec![],
            level: 0
        }
    }

    fn new_with_value(key: SExpr, val: SExpr) -> Binding {
        Binding {
            key,
            vec: vec![(0, val)],
            level: 0
        }
    }

    fn add_val(&mut self, val: SExpr) {
        self.vec.push((self.level, val));
    }

    fn up_level(&mut self) {
        self.level += 1;
    }
}

pub fn pattern_matches(pattern: SExpr, candidate: SExpr) -> Option<BindingBuilder> {
        fn bind(expr_pat: SExpr, expr_can: SExpr, bindings: &mut BindingBuilder) -> bool {
            match (expr_pat, expr_can) {
                (p@SExpr::Atom(Token::Symbol(_)), c@SExpr::Atom(_))
                    | (p@SExpr::Atom(Token::Symbol(_)), c@SExpr::List(_)) => {
                    bindings.put(p, c);
                    true
                },
                (SExpr::List(ps), SExpr::List(cs)) => {
                    for pi in 0..ps.len() {
                        if ps[pi].is_ellipsis() {
                            for ci in pi..cs.len() {
                                if !bind(ps[pi-1].clone(), cs[ci].clone(), bindings) {
                                    return false
                                }
                            }
                        } else {
                            if !bind(ps[pi].clone(), cs[pi].clone(), bindings) {
                                return false
                            }
                        }
                    }

                    true
                },
                _ => false
            }
        }

        let mut bindings = BindingBuilder::new();
        if bind(pattern, candidate, &mut bindings) {
            Some(bindings)
        } else {
            None
        }
}



























