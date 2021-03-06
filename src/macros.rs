use std::mem::ManuallyDrop;

macro_rules! defer {
    ($action:expr) => {
        let _defer = crate::macros::RunOnDrop::new(move || {
            $action;
        });
    };
}

pub(crate) struct RunOnDrop<F: FnOnce()>(ManuallyDrop<F>);

impl<F: FnOnce()> RunOnDrop<F> {
    pub(crate) fn new(f: F) -> Self {
        Self(ManuallyDrop::new(f))
    }
}

impl<F: FnOnce()> Drop for RunOnDrop<F> {
    fn drop(&mut self) {
        // Safety: self is being dropped, so it can't be used again
        let f = unsafe { ManuallyDrop::take(&mut self.0) };
        f();
    }
}
