mod declaration;
mod expression;
mod statement;
mod util;
use crate::parser::declaration::*;
use crate::parser::statement::*;
use crate::parser::util::*;
use crate::tokenizer::*;

#[derive(Debug)]
pub struct AST<'a> {
    root: StatementNode<'a>,
}

pub fn parse<'a>(tokens: &'a [Token<'a>]) -> Result<AST<'a>, String> {
    let mut context = ParseContext::new(tokens);
    let node = statement(&mut context)?;
    Ok(AST { root: node })
}
