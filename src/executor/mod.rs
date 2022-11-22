pub mod nonblock_executor;

use crate::ThreadWaker;

use std::{
    future::Future,
    fmt::Debug,
    sync::Arc,
    thread::{park},
    task::{Poll, Context}
};


pub struct BlockingExecutor;

impl BlockingExecutor {
    pub fn run<F>(f: F) -> F::Output
    where
        F: Future,
        F::Output: Debug,
    {
        let mut boxed_future = Box::pin(f);
        let waker = Arc::new(ThreadWaker {
            thread: std::thread::current(),
        })
        .into();
        let mut cx = Context::from_waker(&waker);

        loop {
            let result = boxed_future.as_mut().poll(&mut cx);
            println!("poll = {:?}", result);
            match result {
                Poll::Ready(res) => return res,
                Poll::Pending => park(),
            }
        }
    }
}
