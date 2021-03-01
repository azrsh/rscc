use crate::parser::expression::*;
use crate::parser::util::*;
use crate::tokenizer::*;

#[derive(Debug)]
pub enum StatementNode<'a> {
    Null,
    Expression(Expression<'a>),
    If {
        condition: Box<Expression<'a>>,
        statement: Box<StatementNode<'a>>,
    },
    Switch {
        condition: Box<Expression<'a>>,
        statement: Box<StatementNode<'a>>,
    },
    Labeled(&'a str),
    While {
        condition: Box<Expression<'a>>,
        statement: Box<StatementNode<'a>>,
    },
    DoWhile {
        condition: Box<Expression<'a>>,
        statement: Box<StatementNode<'a>>,
    },
    For {
        initialization: Box<Expression<'a>>,
        condition: Box<Expression<'a>>,
        afterthought: Box<Expression<'a>>,
        statement: Box<StatementNode<'a>>,
    },
    Compound(Vec<StatementNode<'a>>),
    Return(Box<Expression<'a>>),
    Break,
    Continue,
    Goto(&'a str),
}

//parser body
pub fn statement<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<StatementNode<'b>, String> {
    if let Some(result) = null_statement(context)? {
        Ok(result)
    } else if let Some(result) = if_statement(context)? {
        Ok(result)
    } else if let Some(result) = switch_statement(context)? {
        Ok(result)
    } else if let Some(result) = labeled_statement(context)? {
        Ok(result)
    } else if let Some(result) = while_statement(context)? {
        Ok(result)
    } else if let Some(result) = do_while_statement(context)? {
        Ok(result)
    } else if let Some(result) = for_statement(context)? {
        Ok(result)
    } else if let Some(result) = compound_statement(context)? {
        Ok(result)
    } else if let Some(result) = return_statement(context)? {
        Ok(result)
    } else if let Some(result) = break_statement(context)? {
        Ok(result)
    } else if let Some(result) = continue_statement(context)? {
        Ok(result)
    } else if let Some(result) = goto_statement(context)? {
        Ok(result)
    } else if let Some(result) = expression_statement(context)? {
        Ok(result)
    } else {
        Err("statement is expected but not found.".to_string())
    }
}

#[allow(clippy::unnecessary_wraps)]
fn null_statement<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<StatementNode<'b>>, String> {
    let result = if consume_punctuator(context, PunctuatorKind::Semicolon).is_some() {
        Some(StatementNode::Null)
    } else {
        None
    };

    Ok(result)
}

fn expression_statement<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<StatementNode<'b>>, String> {
    let result = expression(context)?;
    expect_punctuator(context, PunctuatorKind::Semicolon)?;
    let result = StatementNode::Expression(result);
    Ok(Some(result))
}

fn if_statement<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<StatementNode<'b>>, String> {
    if consume_keyword(context, KeywordKind::If).is_none() {
        return Ok(None);
    }

    expect_punctuator(context, PunctuatorKind::LeftRoundBracket)?;
    let cond = expression(context)?;
    expect_punctuator(context, PunctuatorKind::RightRoundBracket)?;

    let body = statement(context)?;

    Ok(Some(StatementNode::If {
        condition: Box::new(cond),
        statement: Box::new(body),
    }))
}

fn switch_statement<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<StatementNode<'b>>, String> {
    if consume_keyword(context, KeywordKind::Switch).is_none() {
        return Ok(None);
    }

    expect_punctuator(context, PunctuatorKind::LeftRoundBracket)?;
    let cond = expression(context)?;
    expect_punctuator(context, PunctuatorKind::RightRoundBracket)?;

    let body = statement(context)?;

    Ok(Some(StatementNode::Switch {
        condition: Box::new(cond),
        statement: Box::new(body),
    }))
}

fn labeled_statement<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<StatementNode<'b>>, String> {
    let label = match consume_identifier(context) {
        Some(result) => {
            expect_punctuator(context, PunctuatorKind::Colon)?;
            StatementNode::Labeled(result)
        }
        None => return Ok(None),
    };
    Ok(Some(label))
}

fn while_statement<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<StatementNode<'b>>, String> {
    if consume_keyword(context, KeywordKind::While).is_none() {
        return Ok(None);
    }

    expect_punctuator(context, PunctuatorKind::LeftRoundBracket)?;
    let cond = expression(context)?;
    expect_punctuator(context, PunctuatorKind::RightRoundBracket)?;

    let body = statement(context)?;

    Ok(Some(StatementNode::While {
        condition: Box::new(cond),
        statement: Box::new(body),
    }))
}
fn do_while_statement<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<StatementNode<'b>>, String> {
    if consume_keyword(context, KeywordKind::Do).is_none() {
        return Ok(None);
    }

    expect_punctuator(context, PunctuatorKind::LeftRoundBracket)?;
    let cond = expression(context)?;
    expect_punctuator(context, PunctuatorKind::RightRoundBracket)?;

    let body = statement(context)?;

    expect_keyword(context, KeywordKind::While)?;

    Ok(Some(StatementNode::DoWhile {
        condition: Box::new(cond),
        statement: Box::new(body),
    }))
}

fn for_statement<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<StatementNode<'b>>, String> {
    if consume_keyword(context, KeywordKind::For).is_none() {
        return Ok(None);
    }

    expect_punctuator(context, PunctuatorKind::LeftRoundBracket)?;
    let init = expression(context)?;
    expect_punctuator(context, PunctuatorKind::Semicolon)?;
    let cond = expression(context)?;
    expect_punctuator(context, PunctuatorKind::Semicolon)?;
    let after = expression(context)?;
    expect_punctuator(context, PunctuatorKind::RightRoundBracket)?;

    let body = statement(context)?;

    Ok(Some(StatementNode::For {
        initialization: Box::new(init),
        condition: Box::new(cond),
        afterthought: Box::new(after),
        statement: Box::new(body),
    }))
}

fn compound_statement<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<StatementNode<'b>>, String> {
    if consume_punctuator(context, PunctuatorKind::LeftCurlyBracket).is_none() {
        return Ok(None);
    }

    let mut result = Vec::new();
    while consume_punctuator(context, PunctuatorKind::RightCurlyBracket).is_none() {
        result.push(statement(context)?);
    }

    Ok(Some(StatementNode::Compound(result)))
}

fn return_statement<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<StatementNode<'b>>, String> {
    if consume_keyword(context, KeywordKind::Return).is_none() {
        return Ok(None);
    }

    let content = expression(context)?;

    Ok(Some(StatementNode::Return(Box::new(content))))
}

fn break_statement<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<StatementNode<'b>>, String> {
    let result = match consume_keyword(context, KeywordKind::Break) {
        Some(_) => {
            expect_punctuator(context, PunctuatorKind::Semicolon)?;
            Some(StatementNode::Break)
        }
        None => None,
    };
    Ok(result)
}

fn continue_statement<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<StatementNode<'b>>, String> {
    let result = match consume_keyword(context, KeywordKind::Continue) {
        Some(_) => {
            expect_punctuator(context, PunctuatorKind::Semicolon)?;
            Some(StatementNode::Continue)
        }
        None => None,
    };
    Ok(result)
}

fn goto_statement<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<StatementNode<'b>>, String> {
    if consume_keyword(context, KeywordKind::Goto).is_none() {
        return Ok(None);
    }

    let label = expect_identifier(context)?;

    Ok(Some(StatementNode::Goto(label)))
}
