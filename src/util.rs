use std::iter::*;
use std::ops::*;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NonEmptyVec<T> {
    body: Vec<T>,
}

impl<T> NonEmptyVec<T> {
    pub fn new(neccesary: T) -> NonEmptyVec<T> {
        NonEmptyVec {
            body: [neccesary].into(),
        }
    }

    pub fn unwrap(self) -> Vec<T> {
        self.body
    }
}

impl<T> From<Vec<T>> for NonEmptyVec<T> {
    fn from(from: Vec<T>) -> NonEmptyVec<T> {
        assert!(!from.is_empty());
        NonEmptyVec { body: from }
    }
}

pub fn vec_to_optional_non_empty_vec<T>(from: Vec<T>) -> Option<NonEmptyVec<T>> {
    if from.is_empty() {
        None
    } else {
        Some(from.into())
    }
}

pub fn optional_non_empty_vec_to_vec<T>(from: Option<NonEmptyVec<T>>) -> Vec<T> {
    match from {
        Some(vec) => vec.body,
        None => Vec::new(),
    }
}

impl<T, U, V> Index<U> for NonEmptyVec<T>
where
    Vec<T>: Index<U, Output = V>,
{
    type Output = V;
    fn index(&self, index: U) -> &V {
        self.body.index(index)
    }
}

impl<T, U, V> IndexMut<U> for NonEmptyVec<T>
where
    Vec<T>: IndexMut<U, Output = V>,
{
    fn index_mut(&mut self, index: U) -> &mut V {
        self.body.index_mut(index)
    }
}

impl<T> Deref for NonEmptyVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        self.body.deref()
    }
}

impl<T> DerefMut for NonEmptyVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.body.deref_mut()
    }
}

impl<T> Extend<T> for NonEmptyVec<T> {
    fn extend<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.body.extend(iterable)
    }
}

impl<'a, T> Extend<&'a T> for NonEmptyVec<T>
where
    T: Copy + 'a,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
    {
        self.body.extend(iter)
    }
}

impl<T> AsRef<[T]> for NonEmptyVec<T> {
    fn as_ref(&self) -> &[T] {
        self.body.as_ref()
    }
}

impl<T> AsMut<[T]> for NonEmptyVec<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.body.as_mut()
    }
}

#[derive(Debug)]
pub enum EitherOrBoth<L, R> {
    Both(L, R),
    Left(L),
    Right(R),
}

#[derive(Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}
