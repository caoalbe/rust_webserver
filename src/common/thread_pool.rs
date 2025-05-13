use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread::{self, JoinHandle},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    producer: Option<Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let (producer, consumer) = mpsc::channel();

        let consumer: Arc<Mutex<Receiver<Job>>> = Arc::new(Mutex::new(consumer));

        for id in 0..size {
            // spawn threads
            workers.push(Worker::new(id, Arc::clone(&consumer)));
        }

        ThreadPool {
            workers,
            producer: Some(producer),
        }
    }

    pub fn execute<F>(&self, closure: F) -> ()
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(closure);
        self.producer.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.producer.take());
        for worker in self.workers.drain(..) {
            worker.thread.join().unwrap();
            #[cfg(debug_assertions)]
            println!("Shutdown worker {}", worker.id);
        }
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, consumer: Arc<Mutex<Receiver<Job>>>) -> Worker {
        Worker {
            id,
            thread: thread::spawn(move || {
                loop {
                    let maybe_job = consumer.lock().unwrap().recv();
                    match maybe_job {
                        Ok(job) => {
                            #[cfg(debug_assertions)]
                            println!("Worker {id} running job");
                            job();
                            #[cfg(debug_assertions)]
                            println!("Worker {id} finished job");
                        }
                        Err(std::sync::mpsc::RecvError) => {
                            // The .recv operation can only fail if the sending half of a
                            // channel is disconnected.
                            // Implying that no further messages will ever be received.
                            #[cfg(debug_assertions)]
                            println!("Worker {id} shutting down");
                            break;
                        }
                    }
                }
            }),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
