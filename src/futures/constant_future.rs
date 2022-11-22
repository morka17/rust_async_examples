
use std::{
    ptr,
    future::Future,
    pin::Pin,
    task::{Poll, Waker, RawWakerVTable, RawWaker, Context},
};

#[derive(Debug)]
pub struct ConstantFuture {
    value: i32,
}

impl ConstantFuture {
    pub fn new(value: i32) -> Self {
        ConstantFuture { value }
    }
}

impl Future for ConstantFuture {
    type Output = i32;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        Poll::Ready(self.value)
    }
}





