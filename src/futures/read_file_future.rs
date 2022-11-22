use std::{
    fs::read_to_string,
    thread::spawn,
    pin::Pin,
    future::Future,
    sync::{Arc, Mutex},
    task::{Poll, Context}
};



pub struct ReadFileFuture {
    path: String,
    state: Arc<Mutex<SharedState>>,
}

pub struct SharedState {
    running: bool,
    contents: Option<String>,
}

impl ReadFileFuture {
    pub fn new(path: &str) -> Self {
        ReadFileFuture {
            path: "text.txt".to_string(),
            state: Arc::new(Mutex::new(SharedState {
                running: false,
                contents: None,
            })),
        }
    }
}

impl Future for ReadFileFuture {
    type Output = String;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();

        if !state.running {
            state.running = true;
            let path = self.path.clone();
            let thread_state = self.state.clone();
            let thread_waker = cx.waker().clone();
            spawn(move || {
                let contents = read_to_string(path).expect("can read file");
                let mut shared_state = thread_state.lock().unwrap();
                shared_state.contents = Some(contents);
                thread_waker.wake();
            });
        }

        if let Some(val) = &state.contents{
            Poll::Ready(val.to_owned())
        }else {
            Poll::Pending
        }
    }
}