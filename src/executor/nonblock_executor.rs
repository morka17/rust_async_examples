use std::{
    ptr,
    future::Future,
    task::{Waker, Poll, RawWaker,  Context, RawWakerVTable},
}; 



pub struct Executor;

impl Executor {
    pub fn run<F: Future>(f: F)-> F::Output{
        let mut boxed_future = Box::pin(f);
        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);
        let result = boxed_future.as_mut().poll(&mut context);
        if let Poll::Ready(a) = result {
            return a;
        }else {
            panic!("This executor doesn't really do anything! you asked it to do too much")
        }
    }
}

fn noop_waker() -> Waker {
    unsafe {
        Waker::from_raw(noop_raw_waker())
    }
}


fn noop_raw_waker() -> RawWaker {
    RawWaker::new(ptr::null(), &RAW_WAKER_VTABLE)
}

unsafe fn noop(_p: *const ()){}
unsafe fn noop_clone(_p: *const()) -> RawWaker{
    noop_raw_waker()
}


const RAW_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    noop_clone, // Return the raw pointer
    noop,
    noop, 
    noop
);