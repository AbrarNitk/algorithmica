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
    pub fn new(value: T) -> Self {
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

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        match self.state.get() {
            RefState::Shared(_) | RefState::Exclusive => None,
            RefState::Unshared => {
                self.state.set(RefState::Exclusive);
                Some(RefMut { refcell: self })
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

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(_) | RefState::Unshared => {
                unreachable!("Having Shared state with Exclusive state is impossible")
            }
            RefState::Exclusive => self.refcell.state.set(RefState::Unshared),
        }
    }
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: No other shared reference have given out
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: No other shared reference have given out
        unsafe { &mut *self.refcell.value.get() }
    }
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

    #[test]
    fn test_2() {
        #[derive(Copy, Clone)]
        struct RefCellTest {
            integer: i32,
        }

        impl RefCellTest {
            fn change_value(&mut self, value: i32) {
                self.integer = value;
            }

            fn print_value(&self) {
                println!("{}", self.integer);
            }
        }

        let refcell = super::RefCell::new(RefCellTest { integer: 10 });
        let mut cell = refcell.borrow_mut();
        if let Some(ref mut c) = cell {
            c.change_value(30);
            c.print_value();
        };
        let cell = refcell.borrow();
        if let Some(c) = cell {
            assert_eq!(c.integer, 30)
        }

        let new_cell = refcell.borrow();
        let new_cell_1 = refcell.borrow_mut();
        match new_cell_1 {
            Some(_) => {
                println!("This is clear error");
            }
            None => {
                println!("Cool!, It is working as expected");
            }
        }
    }
}
