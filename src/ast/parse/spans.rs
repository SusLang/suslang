use std::path::Path;

use nom::{character::complete::char, combinator::consumed, error::ParseError, Parser};
use nom_locate::LocatedSpan;

use super::error::IResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraData<'a, T> {
    pub filename: &'a Path,
    pub file_contents: &'a str,
    pub data: T,
}

pub trait MapExt: Sized {
    type Input;
    type Output<T>;

    fn map<T, F: FnOnce(Self::Input) -> T>(self, f: F) -> Self::Output<T>;
}

impl<'a, D> MapExt for ExtraData<'a, D> {
    type Input = D;

    type Output<T> = ExtraData<'a, T>;

    fn map<T, F: FnOnce(Self::Input) -> T>(self, f: F) -> Self::Output<T> {
        Self::Output {
            data: f(self.data),
            filename: self.filename,
            file_contents: self.file_contents,
        }
    }
}

impl<'a, E> MapExt for Span<'a, E> {
    type Input = E;
    type Output<T> = Span<'a, T>;

    fn map<T, F: FnOnce(Self::Input) -> T>(self, f: F) -> Self::Output<T> {
        unsafe {
            Span::new_from_raw_offset(
                self.location_offset(),
                self.location_line(),
                self.fragment(),
                self.extra.map(f),
            )
        }
    }
}

pub type Span<'a, T = ()> = LocatedSpan<&'a str, ExtraData<'a, T>>;

pub fn load_file_str<'a, P: AsRef<Path>>(path: &'a P, contents: &'a str) -> Span<'a> {
    Span::new_extra(
        contents,
        ExtraData {
            filename: path.as_ref(),
            file_contents: contents,
            data: (),
        },
    )
}

pub fn spanned<'a, E: ParseError<Span<'a>>, O, P: Parser<Span<'a>, O, E>>(
    parser: P,
) -> impl FnMut(Span<'a>) -> IResult<'a, E, O> {
    nom::combinator::map(consumed(parser), |(span, o)| span.map(|()| o))
}

pub fn spanned_char<'a, E: ParseError<Span<'a>>>(
    c: char,
) -> impl FnMut(Span<'a>) -> IResult<'a, E, char> {
    spanned(char(c))
}

pub fn spanned_map<'a, I, O1, O2, E, F, G>(
    mut parser: F,
    f: G,
) -> impl FnMut(Span<'a, I>) -> nom::IResult<Span<'a, I>, Span<'a, O2>, E>
where
    F: Parser<Span<'a, I>, Span<'a, O1>, E>,
    G: FnMut(O1) -> O2 + Clone,
{
    move |input: Span<'a, I>| {
        let (input, o1) = parser.parse(input)?;
        Ok((input, o1.map(f.clone())))
    }
}

pub fn spanned_value<'a, T: Clone + 'a, U, E, P: Parser<Span<'a>, Span<'a, U>, E>>(
    value: &'a T,
    parser: P,
) -> impl FnMut(Span<'a>) -> IResult<'a, E, T> {
    nom::combinator::map(parser, |span| span.map(|_| value.clone()))
}
