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
