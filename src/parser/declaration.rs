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
    let mut declaration_specifiers = Vec::new();
    while let Some(specifier) = declaration_specifier(context)? {
        declaration_specifiers.push(specifier);
    }
    if declaration_specifiers.is_empty() {
        return Ok(None);
    }
    let declaration_specifiers = declaration_specifiers.into();

    let mut init_declarators = Vec::new();
    while let Some(declarator) = init_declarator(context)? {
        init_declarators.push(declarator);
    }

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
            specifier
        } else if let Some(specifier) = type_specifier(context)? {
            specifier
        } else if let Some(specifier) = type_qualifier(context)? {
            specifier
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
) -> Result<Option<DeclarationSpecifier<'b>>, String> {
    Err("".to_string())
}

fn type_specifier<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<DeclarationSpecifier<'b>>, String> {
    Err("".to_string())
}

fn type_qualifier<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<Option<DeclarationSpecifier<'b>>, String> {
    Err("".to_string())
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
