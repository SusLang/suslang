use std::{collections::HashMap, marker::PhantomData, fmt::Debug};

pub trait Scope<'a, T>: Debug {
	fn add(&mut self, n: &'a str, t: T);

	fn get(&self, n: &str) -> Option<&T>;

	fn push<'b>(&'b mut self) -> ChildScope<'a, 'b, T>;
}

#[derive(Debug)]
pub struct GlobalScope<'a, T> {
	data: HashMap<&'a str, T>
}

impl<'a, T> GlobalScope<'a, T> {
	pub fn new() -> Self {
		Self::default()
	}
}

impl<'a, T> Default for GlobalScope<'a, T> {
    fn default() -> Self {
        Self { data: Default::default() }
    }
}

impl<'a, T> Scope<'a, T> for GlobalScope<'a, T> where T: Debug {
    fn add(&mut self, n: &'a str, t: T) {
        self.data.insert(n, t);
    }

    fn get(&self, n: &str) -> Option<&T> {
        self.data.get(n)
    }

    fn push<'b>(&'b mut self) -> ChildScope<'a, 'b, T> {
        ChildScope::new(self)
    }
}

pub struct ChildScope<'a, 'b, T> where 'a: 'b {
	parent: &'b mut dyn Scope<'a, T>,
	data: HashMap<&'b str, T>,
	phantom: PhantomData<&'a ()>
}

impl<'a, 'b, T> ChildScope<'a, 'b, T> {
	fn new(parent: &'b mut dyn Scope<'a, T>) -> Self {
		Self { parent, data: Default::default(), phantom: PhantomData }
	}
}

impl<'a, 'b, T> Scope<'b, T> for ChildScope<'a, 'b, T> where T: Debug {
    fn add(&mut self, n: &'b str, t: T) {
        self.data.insert(n, t);
    }

    fn get(&self, n: &str) -> Option<&T> {
        self.data.get(n).or_else(|| self.parent.get(n))
    }

    fn push<'c>(&'c mut self) -> ChildScope<'b, 'c, T> {
        ChildScope::new(self)
    }
}

impl<'a, 'b, T> Debug for ChildScope<'a, 'b, T> where T: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChildScope").field("parent", &self.parent).field("data", &self.data).finish()
    }
}


