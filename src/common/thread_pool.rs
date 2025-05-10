use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread::{self, JoinHandle},
};

pub struct ThreadPool {
    _workers: Vec<Worker>,
    producer: Sender<Job>,
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

        ThreadPool { _workers: workers, producer }
    }

    pub fn execute<F>(&self, closure: F) -> ()
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(closure);
        self.producer.send(job).unwrap();
    }
}

struct Worker {
    _id: usize,
    _thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, producer: Arc<Mutex<Receiver<Job>>>) -> Worker {
        Worker {
            _id: id,
            _thread: thread::spawn(move || {
                loop {
                    let job: Box<dyn FnOnce() + Send> = producer.lock().unwrap().recv().unwrap();
                    println!("Worker {id} running job");
                    job();
                    println!("Worker {id} finished job")
                }
            }),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
