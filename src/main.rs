mod executor;
mod wakers;
mod futures;

pub use wakers::thread_waker::ThreadWaker;
pub use futures::{
    constant_future::ConstantFuture,
    read_file_future::ReadFileFuture,
    timer_future::TimerFuture,
};
pub use executor::{
    BlockingExecutor,
    nonblock_executor::Executor,
};

use std::{
    fs::read_to_string,
};


fn main() {
    //  constant Future
    let future = ConstantFuture::new(5);
    println!("future {:?}", future);

    // Non blocking execution 
    let result = Executor::run(future);
    println!("result {}", result);

    // blocking execution
    let result = BlockingExecutor::run(
        // read file future
        ReadFileFuture::new("text.txt")
    );
    println!("result = {}", result);
}

async fn read_file() -> String {
    let contents = read_to_string("text.txt");
    contents.expect("Error opening file")
}

