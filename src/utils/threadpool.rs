// based on the code of this tutorial:
// https://doc.rust-lang.org/1.25.0/book/second-edition/ch20-03-designing-the-interface.html

use std::{
    sync::{
        mpsc::{Receiver, Sender, channel},
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
    Die
}

struct Worker {
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let mess = receiver.lock().unwrap().recv().unwrap();

            match mess {
                Message::Die => break,
                Message::Execute(job) => job.call_box()
            }
        });

        Self { thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

impl ThreadPool {

    pub fn new(size: usize) -> Self {
        assert!(size > 0, "Tried to create a ThreadPool without threads. :(");

        let (sender, receiver) = channel();
        let atomic_receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&atomic_receiver)));
        }

        Self {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, function: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(function);
        self.sender.send(Message::Execute(job)).unwrap();
    }

}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Die).unwrap();
        }
        while let Some(worker) = self.workers.pop() {
            worker.thread.join().unwrap();
        }
    }
}
