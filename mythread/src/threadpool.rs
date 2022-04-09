#![allow(dead_code, unused_imports)]
use std::sync::mpsc;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool{
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool{
    pub fn new(size: usize) -> ThreadPool{
        assert!(size>0);

        let (sender,receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut threads = Vec::with_capacity(size);

        for id in 0..size{
            threads.push(Worker::new(id,Arc::clone(&receiver)));
        }

        ThreadPool{ threads,sender }
    }

    pub fn execute<F>(&self, f:F)
        where
            F: FnOnce()+Send+'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker{
    id : usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker{
    fn new(id: usize,receiver: Arc<Mutex<mpsc::Receiver<Job>>>)-> Worker{
        let thread = thread::spawn(move||{
                loop{
                    let job = receiver.lock().unwrap().recv().unwrap();
                    println!("Worker {} got a job  ",id);
                    job();
                }
        });

        Worker{
            id , 
            thread: Some(thread),
        }
    }
}