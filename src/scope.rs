use std::{collections::HashMap, fmt::Debug, hash::Hash, marker::PhantomData};

pub trait Scope<T, K>: Debug {
    fn add(&mut self, n: K, t: T);

    fn get(&self, n: &K) -> Option<&T>;

    fn push(&mut self) -> ChildScope<T, K>;
}

#[derive(Debug)]
pub struct GlobalScope<'a, T, K = &'a str> {
    data: HashMap<K, T>,
    _p: PhantomData<&'a str>,
}

impl<'a, T, K> GlobalScope<'a, T, K> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a, T, K> Default for GlobalScope<'a, T, K> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            _p: PhantomData,
        }
    }
}

impl<'a, T, K> Scope<T, K> for GlobalScope<'a, T, K>
where
    T: Debug,
    K: Debug + Eq + Hash,
{
    fn add(&mut self, n: K, t: T) {
        self.data.insert(n, t);
    }

    fn get(&self, n: &K) -> Option<&T> {
        self.data.get(n)
    }

    fn push(&mut self) -> ChildScope<T, K> {
        ChildScope::new(self)
    }
}

pub struct ChildScope<'b, T, K> {
    parent: &'b mut dyn Scope<T, K>,
    data: HashMap<K, T>,
}

impl<'b, T, K> ChildScope<'b, T, K> {
    fn new(parent: &'b mut dyn Scope<T, K>) -> Self {
        Self {
            parent,
            data: Default::default(),
        }
    }
}

impl<'b, T, K> Scope<T, K> for ChildScope<'b, T, K>
where
    T: Debug,
    K: Eq + Hash + Debug,
{
    fn add(&mut self, n: K, t: T) {
        self.data.insert(n, t);
    }

    fn get(&self, n: &K) -> Option<&T> {
        self.data.get(n).or_else(|| self.parent.get(n))
    }

    fn push(&mut self) -> ChildScope<T, K> {
        ChildScope::new(self)
    }
}

impl<'b, T, K> Debug for ChildScope<'b, T, K>
where
    T: Debug,
    K: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChildScope")
            .field("parent", &self.parent)
            .field("data", &self.data)
            .finish()
    }
}
