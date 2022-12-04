use std::path::Path;

use nom_locate::LocatedSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraData<'a, T> {
    pub filename: &'a Path,
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
            data: (),
        },
    )
}
