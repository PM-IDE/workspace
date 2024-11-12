use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;

pub enum ReferenceOrOwned<'a, T> {
    Ref(&'a T),
    Owned(T),
}

impl<'a, T> Deref for ReferenceOrOwned<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            ReferenceOrOwned::Ref(reference) => reference,
            ReferenceOrOwned::Owned(value) => &value,
        }
    }
}

pub enum HeapedOrOwned<T> {
    Heaped(Rc<Box<T>>),
    Owned(T),
}

impl<T> Deref for HeapedOrOwned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            HeapedOrOwned::Heaped(ptr) => ptr.as_ref().as_ref(),
            HeapedOrOwned::Owned(value) => &value,
        }
    }
}

impl<T> Display for HeapedOrOwned<T>
where
    T: ToString,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            HeapedOrOwned::Heaped(heaped) => heaped.as_ref().as_ref().to_string(),
            HeapedOrOwned::Owned(owned) => owned.to_string(),
        };

        write!(f, "{}", str)
    }
}

impl<T> Hash for HeapedOrOwned<T>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            HeapedOrOwned::Heaped(heaped) => heaped.as_ref().as_ref().hash(state),
            HeapedOrOwned::Owned(owned) => owned.hash(state),
        }
    }
}

impl<T> PartialEq for HeapedOrOwned<T>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HeapedOrOwned::Owned(this), HeapedOrOwned::Owned(other)) => this.eq(other),
            (HeapedOrOwned::Heaped(this), HeapedOrOwned::Heaped(other)) => this.eq(other),
            _ => false,
        }
    }
}

impl<T> Eq for HeapedOrOwned<T> where T: Eq {}

impl<T> Clone for HeapedOrOwned<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            HeapedOrOwned::Heaped(heaped) => HeapedOrOwned::Heaped(heaped.clone()),
            HeapedOrOwned::Owned(owned) => HeapedOrOwned::Owned(owned.clone()),
        }
    }
}
