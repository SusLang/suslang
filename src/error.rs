use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

use crate::{ast::parse::spans::Span, typecheck};

#[derive(Error, Debug, Diagnostic)]
pub enum TypeCheckError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    ModuleNotFound(#[from] ModuleNotFoundError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    ItemNotFound(#[from] ItemNotFound),
    #[error(transparent)]
    #[diagnostic(transparent)]
    ExpressionTypeError(#[from] ExpressionTypeError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    FunctionNotFound(#[from] FunctionNotFound),
    #[error(transparent)]
    #[diagnostic(transparent)]
    FunctionArgumentNumber(#[from] FunctionArgumentNumber),
    #[error(transparent)]
    #[diagnostic(transparent)]
    FunctionArgumentTypeError(#[from] FunctionArgumentTypeError),
}

#[derive(Error, Debug, Diagnostic)]
#[error("Module not found")]
#[diagnostic(code(suslang::module::not_found), url(docsrs))]
pub struct ModuleNotFoundError {
    #[source_code]
    src: NamedSource,
    #[label]
    bad_bit: SourceSpan,
}

impl<'a> From<Span<'a>> for ModuleNotFoundError {
    fn from(value: Span<'a>) -> Self {
        Self {
            src: NamedSource::new(
                value.extra.filename.display().to_string(),
                value.extra.file_contents.to_string(),
            ),
            bad_bit: (value.location_offset(), value.len()).into(),
        }
    }
}

#[derive(Error, Debug, Diagnostic)]
#[error("Item `{item}` not found")]
#[diagnostic(code(suslang::item::not_found), url(docsrs), severity = "error")]
pub struct ItemNotFound {
    #[source_code]
    src: NamedSource,
    #[label]
    bad_bit: SourceSpan,
    item: String,
}

impl<'a> From<Span<'a, &str>> for ItemNotFound {
    fn from(value: Span<'a, &str>) -> Self {
        Self {
            src: NamedSource::new(
                value.extra.filename.display().to_string(),
                value.extra.file_contents.to_string(),
            ),
            bad_bit: (value.location_offset(), value.len()).into(),
            item: value.extra.data.to_string(),
        }
    }
}

#[derive(Error, Debug, Diagnostic)]
#[error("Expected type {type_found:?} on expression {type_expected:?}")]
#[diagnostic(code(suslang::expression::type_error), url(docsrs))]
pub struct ExpressionTypeError {
    #[source_code]
    src: NamedSource,
    #[label]
    bad_bit: SourceSpan,
    type_found: typecheck::Type,
    type_expected: typecheck::Type,
}

impl<'a> From<Span<'a, (typecheck::Type, typecheck::Type)>> for ExpressionTypeError {
    fn from(value: Span<'a, (typecheck::Type, typecheck::Type)>) -> Self {
        Self {
            src: NamedSource::new(
                value.extra.filename.display().to_string(),
                value.extra.file_contents.to_string(),
            ),
            bad_bit: (value.location_offset(), value.len()).into(),
            type_found: value.extra.data.0,
            type_expected: value.extra.data.1,
        }
    }
}

#[derive(Error, Debug, Diagnostic)]
#[error("Function `{item}` not found")]
#[diagnostic(code(suslang::function::not_found), url(docsrs), severity = "error")]
pub struct FunctionNotFound {
    #[source_code]
    src: NamedSource,
    #[label]
    bad_bit: SourceSpan,
    item: String,
}

impl<'a> From<Span<'a, String>> for FunctionNotFound {
    fn from(value: Span<'a, String>) -> Self {
        Self {
            src: NamedSource::new(
                value.extra.filename.display().to_string(),
                value.extra.file_contents.to_string(),
            ),
            bad_bit: (value.location_offset(), value.len()).into(),
            item: value.extra.data,
        }
    }
}

#[derive(Error, Debug, Diagnostic)]
#[error("Function `{item}` expected {expected} argmuents but found {found} not found")]
#[diagnostic(code(suslang::function::argument::number), url(docsrs), severity = "error")]
pub struct FunctionArgumentNumber {
    #[source_code]
    src: NamedSource,
    #[label]
    bad_bit: SourceSpan,
    item: String,
    expected: usize,
    found: usize
}

impl<'a> From<Span<'a, (String, usize, usize)>> for FunctionArgumentNumber {
    fn from(value: Span<'a, (String, usize, usize)>) -> Self {
        Self {
            src: NamedSource::new(
                value.extra.filename.display().to_string(),
                value.extra.file_contents.to_string(),
            ),
            bad_bit: (value.location_offset(), value.len()).into(),
            item: value.extra.data.0,
            expected: value.extra.data.1,
            found: value.extra.data.2
        }
    }
}

#[derive(Error, Debug, Diagnostic)]
#[error("Function `{function_name}` call argument type error")]
#[diagnostic(code(suslang::function::argument::type_error), url(docsrs), severity = "error")]
pub struct FunctionArgumentTypeError {
    #[related]
    pub related : Vec<ExpressionTypeError>,
    pub function_name: String,
}
