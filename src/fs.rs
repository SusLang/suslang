use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Filesystem<'a>
where
    Self: 'a,
{
    files: Vec<(*mut Path, *mut str)>,
    lifetime: PhantomData<&'a ()>,
}

// TODO is it safe?

impl<'a> Filesystem<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load<'c, 'b>(&'b mut self, p: PathBuf) -> std::io::Result<(&'c Path, &'c str)>
    where
        'a: 'c,
    {
        let s = std::fs::read_to_string(&p)?;
        let p = Box::into_raw(p.into_boxed_path());
        let b = Box::into_raw(s.into_boxed_str());

        self.files.push((p, b));
        let (p, b) = unsafe { (Box::from_raw(p), Box::from_raw(b)) };
        Ok((&*Box::leak(p), &*Box::leak(b)))
    }
}

impl<'a> Drop for Filesystem<'a> {
    fn drop(&mut self) {
        while !self.files.is_empty() {
            let (p, c) = self.files.pop().unwrap();
            let (p, b) = unsafe { (Box::from_raw(p), Box::from_raw(c)) };
            drop(p);
            drop(b);
        }
    }
}
