
use std::{
    thread::Thread,
    sync::{Arc},
    task::Wake,
};


pub struct ThreadWaker {
    pub thread: Thread,
}
impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.thread.unpark();
    }
}
