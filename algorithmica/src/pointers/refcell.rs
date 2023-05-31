// RefCell is used for shared mutation with dynamic borrow checking rules

#[derive(Copy, Clone)]
pub enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

pub struct RefCell<T> {
    value: std::cell::UnsafeCell<T>,
    state: crate::pointers::cell::Cell<RefState>,
}

impl<T> RefCell<T> {
    fn new(value: T) -> Self {
        Self {
            value: std::cell::UnsafeCell::new(value),
            state: crate::pointers::cell::Cell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Exclusive => None,
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                Some(Ref { refcell: self })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(Ref { refcell: self })
            }
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive | RefState::Unshared => {
                unreachable!("Having Exclusive state with Shared reference, this is impossible")
            }
            RefState::Shared(1) => self.refcell.state.set(RefState::Unshared),
            RefState::Shared(n) => self.refcell.state.set(RefState::Shared(n - 1)),
        }
    }
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: No other Exclusive reference have given out
        unsafe { &*self.refcell.value.get() }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {
        let refcell = super::RefCell::new(10);
        let t = match refcell.borrow() {
            Some(r) => *r, // in this it call the deref trait
            None => 1,
        };
        assert_eq!(t, 10)
    }
}
