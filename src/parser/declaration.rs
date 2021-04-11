use crate::parser::expression::*;
use crate::parser::util::*;
use crate::tokenizer::*;
use crate::util::*;

#[derive(Debug)]
pub struct Declaration<'a> {
    specifiers: NonEmptyVec<DeclarationSpecifier<'a>>,
    declarators: Vec<InitDeclarator<'a>>,
}

#[derive(Debug)]
pub enum DeclarationSpecifier<'a> {
    StorageSpecifier(StorageClassSpecifier),
    TypeSpecifier(TypeSpecifier<'a>),
    TypeQualifier(TypeQualifier),
    FunctionSpecifier(FunctionSpecifier),
    AlignmentSpecifier(Either<Type, Expression<'a>>),
}

#[derive(Debug, Clone, Copy)]
pub enum StorageClassSpecifier {
    Typedef,
    Extern,
    Static,
    ThreadLocal,
    Auto,
    Register,
}

#[derive(Debug)]
pub enum TypeSpecifier<'a> {
    Void,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Singned,
    Unsigned,
    Bool,
    Complex,
    AtomicTypeSpecifier,
    StructOrUnionSpecifier(StructOrUnionSpecifier<'a>),
    EnumSpecifier(EnumSpecifier<'a>),
    TypedefName(&'a str),
}

#[derive(Debug, Clone, Copy)]
pub enum TypeQualifier {
    Const,
    Restrict,
    Volatile,
    Atomic,
}

#[derive(Debug, Clone, Copy)]
pub enum FunctionSpecifier {
    Inline,
    Noreturn,
}

#[derive(Debug)]
pub struct StructOrUnionSpecifier<'a> {
    kind: StructOrUnion,
    name_or_menbers: EitherOrBoth<&'a str, NonEmptyVec<&'a str>>,
}

#[derive(Debug, Clone, Copy)]
pub enum StructOrUnion {
    Struct,
    Union,
}

#[derive(Debug)]
pub struct StructDeclaration<'a> {
    specifier_qualifier_list: NonEmptyVec<SpecifierOrQualifier<'a>>,
    declarators: Vec<StructDeclarator<'a>>,
}

#[derive(Debug)]
pub enum SpecifierOrQualifier<'a> {
    TypeSpecifier(TypeSpecifier<'a>),
    TypeQualifier(TypeQualifier),
}

pub type StructDeclarator<'a> = EitherOrBoth<Declarator<'a>, Expression<'a>>;

#[derive(Debug)]
pub struct EnumSpecifier<'a> {
    name_or_menbers: EitherOrBoth<&'a str, NonEmptyVec<&'a str>>,
}

#[derive(Debug)]
pub struct InitDeclarator<'a> {
    declarator: Declarator<'a>,
    initializer: Expression<'a>,
}

#[derive(Debug)]
pub struct Declarator<'a> {
    pointer: Pointer,
    identifier: DirectDeclarator<'a>,
}

pub type Pointer = Vec<Star>;

pub type Star = Vec<TypeQualifier>;

#[derive(Debug)]
pub enum DirectDeclarator<'a> {
    Identifier(&'a str),
    Declarator(Box<Declarator<'a>>),
    Array(
        Box<DirectDeclarator<'a>>,
        Vec<TypeQualifier>,
        Option<Expression<'a>>,
    ),
    Function(
        Box<DirectDeclarator<'a>>,
        Either<ParameterTypeList<'a>, IdentifierList<'a>>,
    ),
}

#[derive(Debug)]
pub struct ParameterTypeList<'a> {
    parameter_list: NonEmptyVec<ParameterDeclaration<'a>>,
    variable_parameter: bool,
}

#[derive(Debug)]
pub struct ParameterDeclaration<'a> {
    specifier: DeclarationSpecifier<'a>,
    declarator: Either<Declarator<'a>, AbstractDeclarator<'a>>,
}

pub type IdentifierList<'a> = Vec<&'a str>;

pub struct TypeName<'a> {
    specifier_qualifier_list: NonEmptyVec<SpecifierOrQualifier<'a>>,
    declarator: Option<AbstractDeclarator<'a>>,
}

#[derive(Debug)]
pub struct AbstractDeclarator<'a> {
    pointer: Pointer,
    declarator: DirectAbstarctDeclarator<'a>,
}

#[derive(Debug)]
pub enum DirectAbstarctDeclarator<'a> {
    Declarator(Box<AbstractDeclarator<'a>>),
    Array(
        Option<Box<DirectAbstarctDeclarator<'a>>>,
        Vec<TypeQualifier>,
        Option<Expression<'a>>,
    ),
    Function(
        Option<Box<DirectAbstarctDeclarator<'a>>>,
        ParameterTypeList<'a>,
    ),
}

//parser body
pub fn declaration<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<Declaration<'b>>, String> {
    let declaration_specifiers = match declaration_specifier(context)? {
        Some(specifier) => specifier,
        None => return Ok(None),
    };

    let mut init_declarators = Vec::new();
    loop {
        let declarator = match init_declarator(context)? {
            Some(d) => d,
            None => return Err("Parse Error".to_string()),
        };
        init_declarators.push(declarator);

        if consume_punctuator(context, PunctuatorKind::Commma).is_none() {
            break;
        }
    }
    expect_punctuator(context, PunctuatorKind::Semicolon)?;

    let result = Declaration {
        specifiers: declaration_specifiers,
        declarators: init_declarators,
    };
    Ok(Some(result))
}

fn declaration_specifier<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<NonEmptyVec<DeclarationSpecifier<'b>>>, String> {
    let mut declaration_specifiers = Vec::new();
    loop {
        let specifier = if let Some(specifier) = storage_class_specifier(context)? {
            DeclarationSpecifier::StorageSpecifier(specifier)
        } else if let Some(specifier) = type_specifier(context)? {
            DeclarationSpecifier::TypeSpecifier(specifier)
        } else if let Some(qualifier) = type_qualifier(context)? {
            DeclarationSpecifier::TypeQualifier(qualifier)
        } else if let Some(specifier) = alignment_specifier(context)? {
            specifier
        } else if let Some(specifier) = function_specifier(context)? {
            specifier
        } else {
            break;
        };

        declaration_specifiers.push(specifier);
    }

    Ok(vec_to_optional_non_empty_vec(declaration_specifiers))
}

fn storage_class_specifier<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<StorageClassSpecifier>, String> {
    let specifier = if consume_keyword(context, KeywordKind::Typedef).is_some() {
        Some(StorageClassSpecifier::Typedef)
    } else if consume_keyword(context, KeywordKind::Extern).is_some() {
        Some(StorageClassSpecifier::Extern)
    } else if consume_keyword(context, KeywordKind::Static).is_some() {
        Some(StorageClassSpecifier::Static)
    } else if consume_keyword(context, KeywordKind::ThreadLocal).is_some() {
        Some(StorageClassSpecifier::ThreadLocal)
    } else if consume_keyword(context, KeywordKind::Auto).is_some() {
        Some(StorageClassSpecifier::Auto)
    } else if consume_keyword(context, KeywordKind::Register).is_some() {
        Some(StorageClassSpecifier::Register)
    } else {
        None
    };

    Ok(specifier)
}

fn type_specifier<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<TypeSpecifier<'b>>, String> {
    let specifier = if consume_keyword(context, KeywordKind::Bool).is_some() {
        Some(TypeSpecifier::Bool)
    } else if consume_keyword(context, KeywordKind::Char).is_some() {
        Some(TypeSpecifier::Char)
    } else if consume_keyword(context, KeywordKind::Short).is_some() {
        Some(TypeSpecifier::Short)
    } else if consume_keyword(context, KeywordKind::Int).is_some() {
        Some(TypeSpecifier::Int)
    } else if consume_keyword(context, KeywordKind::Long).is_some() {
        Some(TypeSpecifier::Long)
    } else if consume_keyword(context, KeywordKind::Float).is_some() {
        Some(TypeSpecifier::Float)
    } else if consume_keyword(context, KeywordKind::Double).is_some() {
        Some(TypeSpecifier::Double)
    } else {
        None
    };

    Ok(specifier)
}

fn type_qualifier<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<TypeQualifier>, String> {
    let qualifier = if consume_keyword(context, KeywordKind::Const).is_some() {
        Some(TypeQualifier::Const)
    } else if consume_keyword(context, KeywordKind::Restrict).is_some() {
        Some(TypeQualifier::Restrict)
    } else if consume_keyword(context, KeywordKind::Volatile).is_some() {
        Some(TypeQualifier::Volatile)
    } else if consume_keyword(context, KeywordKind::Atomic).is_some() {
        Some(TypeQualifier::Atomic)
    } else {
        None
    };

    Ok(qualifier)
}

fn function_specifier<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<DeclarationSpecifier<'b>>, String> {
    Err("".to_string())
}

fn alignment_specifier<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<DeclarationSpecifier<'b>>, String> {
    Err("".to_string())
}

fn init_declarator<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<InitDeclarator<'b>>, String> {
    Err("".to_string())
}

fn declarator<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<Option<Declarator<'b>>, String> {
    Err("".to_string())
}
