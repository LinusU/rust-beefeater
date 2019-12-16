//! Generic guard around any type that implements Copy

pub mod ops;
pub use ops::*;

use std::fmt;
use std::mem;
use std::sync::Mutex;

/// A generic guard around any type that implements Copy, which allows an object to be safely shared between threads.
pub struct Beefeater<T: Copy>(Mutex<T>);

// Beefeater<T> is Sync if T is Send
unsafe impl<T: Copy + Send> Sync for Beefeater<T> {}

impl<T: Copy> Beefeater<T> {
    /// Creates a new `Beefeater`.
    #[inline]
    pub fn new(value: T) -> Beefeater<T> {
        Beefeater(Mutex::new(value))
    }

    /// Loads the current value from the `Beefeater`.
    #[inline]
    pub fn load(&self) -> T {
        *self.0.lock().unwrap()
    }

    /// Stores a new value into the `Beefeater`.
    #[inline]
    pub fn store(&self, val: T) {
        *self.0.lock().unwrap() = val;
    }

    /// Store a new value into the `Beefeater`, returning the old value.
    #[inline]
    pub fn swap(&self, val: T) -> T {
        mem::replace(&mut *self.0.lock().unwrap(), val)
    }
}

impl<T: Copy + Default> Default for Beefeater<T> {
    #[inline]
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T: Copy + fmt::Debug> fmt::Debug for Beefeater<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Beefeater").field(&self.load()).finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::Beefeater;

    #[test]
    fn it_guards_u64() {
        let guard = Beefeater::new(10);
        assert_eq!(guard.load(), 10);
        assert_eq!(guard.load(), 10);

        guard.store(4);
        assert_eq!(guard.load(), 4);
        assert_eq!(guard.load(), 4);
    }

    #[test]
    fn it_guards_enum() {
        #[derive(Clone, Copy, Debug, PartialEq)]
        enum Test {
            A,
            B,
            C,
        }

        let guard = Beefeater::new(Test::B);
        assert_eq!(guard.load(), Test::B);
        assert_eq!(guard.load(), Test::B);

        guard.store(Test::A);
        assert_eq!(guard.load(), Test::A);
        assert_eq!(guard.load(), Test::A);

        guard.store(Test::C);
        assert_eq!(guard.load(), Test::C);
        assert_eq!(guard.load(), Test::C);
    }

    #[test]
    fn it_replaces_value() {
        #[derive(Clone, Copy, Debug, PartialEq)]
        enum Test {
            A,
            B,
            C,
        }

        let guard = Beefeater::new(Test::A);
        assert_eq!(guard.load(), Test::A);

        assert_eq!(guard.swap(Test::B), Test::A);
        assert_eq!(guard.load(), Test::B);

        assert_eq!(guard.swap(Test::C), Test::B);
        assert_eq!(guard.load(), Test::C);

        assert_eq!(guard.swap(Test::C), Test::C);
        assert_eq!(guard.load(), Test::C);
    }

    #[test]
    fn it_implements_default() {
        #[derive(Clone, Copy, Debug, PartialEq)]
        enum Test {
            A,
            B,
            C,
        }

        impl Default for Test {
            fn default() -> Test {
                Test::B
            }
        }

        let guard = Beefeater::default();
        assert_eq!(guard.load(), Test::B);
        assert_eq!(guard.load(), Test::B);

        guard.store(Test::A);
        assert_eq!(guard.load(), Test::A);
        assert_eq!(guard.load(), Test::A);

        guard.store(Test::C);
        assert_eq!(guard.load(), Test::C);
        assert_eq!(guard.load(), Test::C);
    }

    #[test]
    fn it_implements_debug() {
        let guard = Beefeater::new(10);
        assert_eq!(format!("{:?}", guard), "Beefeater(10)");
    }
}
