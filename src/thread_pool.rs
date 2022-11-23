use std::{
    marker::Send,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{spawn, JoinHandle},
};


enum Message {
    NewJob(Job),
    Terminate,
}

// trait FnBox {
//     fn call_box(self: Box<Self>);
// }

// impl<F: FnOnce()> FnBox for F {
//     fn call_box(self: Box<F>){
//         (*self)()
//     }
// }


struct FnBox<F> 
where
    F: FnOnce()
{
    f: F
}

impl<F> FnBox<F> 
where
 F: FnOnce() + Send + 'static 
{
    fn new(&mut self, f: F){
        self.f = f;
    }
    fn call_box(&self){
        (self.f)()
    }
}




type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static
        {
            let job = Box::new(f);

            self.sender.send(Message::NewJob(job)).unwrap()
        }
}


impl Drop for ThreadPool{
    fn drop(&mut self){
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
} 


struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        F
                    },
                    Message::Terminate => {
                        println!("worker {} was told  to terminate.", id);
                        break;
                    }
                }

           
            }
        });

        Worker { id, thread: Some(thread) }
    }
}
