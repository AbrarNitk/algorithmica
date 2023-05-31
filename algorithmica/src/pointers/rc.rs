pub struct RcInner<T> {
    value: T,
    refcount: std::cell::Cell<usize>,
}

pub struct Rc<T> {
    inner: std::ptr::NonNull<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            value,
            refcount: std::cell::Cell::new(1),
        });
        let inner_ptr = Box::into_raw(inner);
        Self {
            // SAFETY: Box never returns null pointer
            inner: unsafe { std::ptr::NonNull::new_unchecked(inner_ptr) },
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Rc { inner: self.inner }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c == 1 {
            // Drop the box here
            // SAFETY: We are only Rc left, so dropping the Box is safe here
            let _ = unsafe { Box::from_raw(self.inner.as_mut()) };
        } else {
            inner.refcount.set(c - 1);
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        let inner = unsafe { self.inner.as_ref() };
        &inner.value
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let rc = super::Rc::new(10);
        let rc_clone = rc.clone();
        assert_eq!(10, *rc_clone)
    }
}
