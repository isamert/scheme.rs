use lexer::Token;
use parser::SExpr;
use serr::{SErr, SResult};

pub fn expand(sexpr: SExpr) -> SResult<SExpr> {
    Ok(sexpr)
    // Expand these:
    // https://schemers.org/Documents/Standards/R5RS/HTML/r5rs-Z-H-6.html#%_sec_3.5
}
