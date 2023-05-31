// It is a shared mutable reference
// Meaning it allowed shared reference to mutate

pub struct Cell<T> {
    value: std::cell::UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: std::cell::UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        let old = self.replace(value);
        drop(old);
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }

    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }

    pub fn replace(&self, value: T) -> T {
        std::mem::replace(unsafe { &mut *self.value.get() }, value)
    }

    pub fn swap(&self, other: &Self) {
        if std::ptr::eq(self, other) {
            return;
        }
        unsafe { std::ptr::swap(self.value.get(), other.value.get()) }
    }

    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(T) -> T,
        T: Copy,
    {
        self.set(f(self.get()));
    }
}

impl<T: Default> Cell<T> {
    pub fn take(&self) -> T {
        self.replace(Default::default())
    }
}

// impl<T> Drop for Cell<T> {
//     fn drop(&mut self) {
//         eprintln!("Dropping Cell Pointer: {:p}", self.value.get());
//     }
// }

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let cell_value = super::Cell::new(10);
        assert_eq!(cell_value.get(), 10)
    }
    #[test]
    fn test_2() {
        let cell_value = super::Cell::new(10);
        cell_value.set(20);
        assert_eq!(cell_value.get(), 20)
    }
}
