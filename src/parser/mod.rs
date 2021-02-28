mod expression;
mod util;
use crate::parser::expression::*;
use crate::parser::util::*;
use crate::tokenizer::*;

#[derive(Debug)]
pub struct AST<'a> {
    root: ExpressionNode<'a>,
}

pub fn parse<'a>(tokens: &'a [Token<'a>]) -> Result<AST<'a>, String> {
    let mut context = ParseContext::new(tokens);
    let node = expression(&mut context)?;
    Ok(AST { root: node })
}
