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
