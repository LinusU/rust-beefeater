use crate::Beefeater;

pub trait AddAssign<Rhs> {
    /// Add to the current value, storing the result.
    fn add_assign(&self, val: Rhs);
}

impl<Lhs: Copy, Rhs> AddAssign<Rhs> for Beefeater<Lhs> where Lhs: std::ops::AddAssign<Rhs> {
    fn add_assign(&self, val: Rhs) {
        self.0.lock().unwrap().add_assign(val)
    }
}

pub trait FetchAdd<Lhs, Rhs> {
    /// Add to the current value, returning the previous value.
    fn fetch_add(&self, val: Rhs) -> Lhs;
}

impl<Lhs: Copy, Rhs> FetchAdd<Lhs, Rhs> for Beefeater<Lhs> where Lhs: std::ops::AddAssign<Rhs> {
    fn fetch_add(&self, val: Rhs) -> Lhs {
        let mut lock = self.0.lock().unwrap();
        let prev = *lock;
        lock.add_assign(val);
        prev
    }
}

pub trait FetchSub<Lhs, Rhs> {
    /// Subtract from the current value, returning the previous value.
    fn fetch_sub(&self, val: Rhs) -> Lhs;
}

impl<Lhs: Copy, Rhs> FetchSub<Lhs, Rhs> for Beefeater<Lhs> where Lhs: std::ops::SubAssign<Rhs> {
    fn fetch_sub(&self, val: Rhs) -> Lhs {
        let mut lock = self.0.lock().unwrap();
        let prev = *lock;
        lock.sub_assign(val);
        prev
    }
}

pub trait SubAssign<Rhs> {
    /// Subtract from the current value, storing the result.
    fn sub_assign(&self, val: Rhs);
}

impl<Lhs: Copy, Rhs> SubAssign<Rhs> for Beefeater<Lhs> where Lhs: std::ops::SubAssign<Rhs> {
    fn sub_assign(&self, val: Rhs) {
        self.0.lock().unwrap().sub_assign(val)
    }
}

#[cfg(test)]
mod tests {
    use crate::Beefeater;

    #[test]
    fn it_implements_add_assign() {
        use crate::ops::AddAssign;

        let guard = Beefeater::new(10);
        assert_eq!(guard.load(), 10);

        guard.add_assign(16);
        assert_eq!(guard.load(), 26);
    }

    #[test]
    fn it_implements_fetch_add() {
        use crate::ops::FetchAdd;

        let guard = Beefeater::new(10);
        assert_eq!(guard.load(), 10);
        assert_eq!(guard.fetch_add(8), 10);
        assert_eq!(guard.fetch_add(8), 18);
        assert_eq!(guard.load(), 26);
    }

    #[test]
    fn it_implements_fetch_sub() {
        use crate::ops::FetchSub;

        let guard = Beefeater::new(10);
        assert_eq!(guard.load(), 10);
        assert_eq!(guard.fetch_sub(2), 10);
        assert_eq!(guard.fetch_sub(2), 8);
        assert_eq!(guard.load(), 6);
    }

    #[test]
    fn it_implements_sub_assign() {
        use crate::ops::SubAssign;

        let guard = Beefeater::new(10);
        assert_eq!(guard.load(), 10);

        guard.sub_assign(4);
        assert_eq!(guard.load(), 6);
    }
}
