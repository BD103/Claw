use std::ops::Deref;
use std::fmt::{self, Debug, Display};

/// A frozed owned smart-pointer.
pub struct P<T: ?Sized> {
    ptr: Box<T>,
}

impl<T: ?Sized> Deref for P<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

// Has to be Sized if not boxed
impl<T> From<T> for P<T> {
    fn from(value: T) -> Self {
        P {
            ptr: Box::new(value),
        }
    }
}

impl<T: ?Sized> From<Box<T>> for P<T> {
    fn from(value: Box<T>) -> Self {
        P {
            ptr: value,
        }
    }
}

impl<T: ?Sized + Debug> Debug for P<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.ptr, f)
    }
}

impl<T: ?Sized + Display> Display for P<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.ptr, f)
    }
}
