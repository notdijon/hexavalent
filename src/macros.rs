use std::mem::ManuallyDrop;
use std::ptr;

macro_rules! defer {
    ($action:expr) => {
        let _defer = crate::macros::RunOnDrop::new(move || {
            $action;
        });
    };
}

pub struct RunOnDrop<F: FnOnce()>(ManuallyDrop<F>);

impl<F: FnOnce()> RunOnDrop<F> {
    pub fn new(f: F) -> Self {
        Self(ManuallyDrop::new(f))
    }
}

impl<F: FnOnce()> Drop for RunOnDrop<F> {
    fn drop(&mut self) {
        // Safety: the value is being dropped, so it can't be used again
        unsafe {
            let f = ManuallyDrop::into_inner(ptr::read(&self.0));
            f();
        }
    }
}

#[cfg(test)]
macro_rules! assert_matches {
    ( $expected:pat, $input:expr ) => {{
        match $input {
            $expected => {}
            ref not_expected => assert!(
                false,
                "{:?} does not match {}",
                not_expected,
                stringify!($expected)
            ),
        }
    }};
}
