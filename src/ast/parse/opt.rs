use nom::{error::ParseError, Err, IResult, Parser};

/// Optional parser: Will return `None` if not successful.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult};
/// use nom::combinator::opt;
/// use nom::character::complete::alpha1;
/// # fn main() {
///
/// fn parser(i: &str) -> IResult<&str, Option<&str>> {
///   opt(alpha1)(i)
/// }
///
/// assert_eq!(parser("abcd;"), Ok((";", Some("abcd"))));
/// assert_eq!(parser("123;"), Ok(("123;", None)));
/// # }
/// ```
pub fn opt<I: Clone, O, E: ParseError<I>, F>(mut f: F) -> impl FnMut(I) -> IResult<I, Option<O>, E>
where
    F: Parser<I, O, E>,
{
    move |input: I| {
        let i = input.clone();
        match f.parse(input) {
            Ok((i, o)) => Ok((i, Some(o))),
            Err(Err::Error(_)) | Err(Err::Incomplete(_)) => Ok((i, None)),
            Err(e) => Err(e),
        }
    }
}
