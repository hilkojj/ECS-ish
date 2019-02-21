// based on the code of this tutorial:
// https://doc.rust-lang.org/1.25.0/book/second-edition/ch20-03-designing-the-interface.html

use std::{
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

enum Message {
    Execute(Job),
    Die,
}

struct Worker {
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(messages: Messages) -> Self {
        let thread = thread::spawn(move || loop {
            if let Some(mess) = messages.lock().unwrap().pop() {
                match mess {
                    Message::Die => break,
                    Message::Execute(job) => job.call_box(),
                }
            }
        });

        Self { thread }
    }
}

type Messages = Arc<Mutex<Vec<Message>>>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    messages: Messages,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let messages = Arc::new(Mutex::new(Vec::new()));
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&messages)));
        }

        Self { workers, messages }
    }

    pub fn execute<F>(&self, function: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if self.workers.is_empty() {
            panic!("This threadpool has no threads");
        }
        let job = Box::new(function);
        self.messages.lock().unwrap().push(Message::Execute(job));
    }

    pub fn idle(&self) -> bool {
        self.messages.lock().unwrap().is_empty()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.messages.lock().unwrap().push(Message::Die);
        }
        while let Some(worker) = self.workers.pop() {
            worker.thread.join().unwrap();
        }
    }
}
