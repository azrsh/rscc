use crate::parser::expression::*;
use crate::parser::util::*;
use crate::tokenizer::*;

#[derive(Debug)]
pub enum StatementNode<'a> {
    NullStatement,
    ExpressionStatement(ExpressionNode<'a>),
    IfStatement {
        condition: Box<ExpressionNode<'a>>,
        statement: Box<StatementNode<'a>>,
    },
    SwitchStatement {
        condition: Box<ExpressionNode<'a>>,
        statement: Box<StatementNode<'a>>,
    },
    LabeledStatement(&'a str),
    WhileStatement {
        condition: Box<ExpressionNode<'a>>,
        statement: Box<StatementNode<'a>>,
    },
    DoWhileStatement {
        condition: Box<ExpressionNode<'a>>,
        statement: Box<StatementNode<'a>>,
    },
    ForStatement {
        initialization: Box<ExpressionNode<'a>>,
        condition: Box<ExpressionNode<'a>>,
        afterthought: Box<ExpressionNode<'a>>,
        statement: Box<StatementNode<'a>>,
    },
    CompoundStatement(Vec<StatementNode<'a>>),
    ReturnStatement(Box<ExpressionNode<'a>>),
    BreakStatement,
    ContinueStatement,
    GotoStatement(&'a str),
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

fn null_statement<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<StatementNode<'b>>, String> {
    let result = if consume_punctuator(context, PunctuatorKind::Semicolon).is_some() {
        Some(StatementNode::NullStatement)
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
    let result = StatementNode::ExpressionStatement(result);
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

    Ok(Some(StatementNode::IfStatement {
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

    Ok(Some(StatementNode::SwitchStatement {
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
            StatementNode::LabeledStatement(result)
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

    Ok(Some(StatementNode::WhileStatement {
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

    Ok(Some(StatementNode::DoWhileStatement {
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

    Ok(Some(StatementNode::ForStatement {
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

    Ok(Some(StatementNode::CompoundStatement(result)))
}

fn return_statement<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<StatementNode<'b>>, String> {
    if consume_keyword(context, KeywordKind::Return).is_none() {
        return Ok(None);
    }

    let content = expression(context)?;

    Ok(Some(StatementNode::ReturnStatement(Box::new(content))))
}

fn break_statement<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<StatementNode<'b>>, String> {
    let result = match consume_keyword(context, KeywordKind::Break) {
        Some(_) => {
            expect_punctuator(context, PunctuatorKind::Semicolon)?;
            Some(StatementNode::BreakStatement)
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
            Some(StatementNode::ContinueStatement)
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

    Ok(Some(StatementNode::GotoStatement(label)))
}
