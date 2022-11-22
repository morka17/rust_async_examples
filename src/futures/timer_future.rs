use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread::{spawn, sleep},
    time::Duration,
};

pub struct TimerFuture {
    /// Future needs a way for the thread to communicate
    /// that the timer has elapsed and the future should complete.
    /// so using a Shared `Arc<Mutex<..>>` value to communicate between the
    /// thread and the future.
    shared_state: Arc<Mutex<SharedState>>,
}

/// Shared state between the future and the waiting thread
struct SharedState {
    /// Whether or not sleep time has elapsed
    completed: bool,

    /// The walker for the task that `TimerFuture` is runnning on.
    /// The thread can use this after setting `completed = true` to tell
    /// `TimerFuture`'s task to wake up,  see that `completed = true`, and 
    /// move forward.
    waker: Option<Waker>,
}

impl Future for TimerFuture{
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx:&mut Context<'_>) -> Poll<Self::Output> {
        // Look at the shared state to see if the timer has already completedd.
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        }else {
            // Set waker to that the htread can wake up the current task
            // when the timer has completed , ensuring that the future is polled
            // again and sees that `completed = true`
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    /// Create a new `TimerFuture` which will complete after the provided
    /// timeout.
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState{
            completed: false,
            waker: None,
        }));

        // Spawn the new thread
        let thread_shared_state = shared_state.clone();
        spawn(move || {
            sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // Signal that the timer has completed and wake up that last
            // task on which the future was polled, if one exists. 
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture {shared_state}
    }
}





















