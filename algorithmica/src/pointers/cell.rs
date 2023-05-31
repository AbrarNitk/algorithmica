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
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

impl<T> Drop for Cell<T> {
    fn drop(&mut self) {
        eprintln!("Dropping Cell Pointer: {:p}", self.value.get());
    }
}

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
