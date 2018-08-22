use parser::SExpr;
use serr::{SResult};

pub fn expand(sexpr: SExpr) -> SResult<SExpr> {
    // TODO: after implementing hygienic macros, expand them here
    Ok(sexpr)
}
