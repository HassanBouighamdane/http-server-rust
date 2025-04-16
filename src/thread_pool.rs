use std::{
    sync::{mpsc::{self, Receiver}, 
    Arc, 
    Mutex}, 
    thread::{self, JoinHandle}
};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender:mpsc::Sender<Job>
}

struct Worker{
    id:usize,
    thread:JoinHandle<()>
}
type Job=Box<dyn FnOnce()+Send+'static>;
impl ThreadPool{
    pub fn new(size:usize)->Self{
        assert!(size>0);
        let mut workers=Vec::with_capacity(size);
        let (sender,receiver)=mpsc::channel();
        let receiver=Arc::new(Mutex::new(receiver));
        for id in 0..size{
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        Self { workers, sender}
    }
    pub fn execute<F>(&self,f:F)
        where 
            F: FnOnce()+Send+'static{
                let job = Box::new(f);
                self.sender.send(job).unwrap();
        }
}

impl Worker{
    fn new(id:usize,receiver:Arc<Mutex<Receiver<Job>>>)->Self{
        let thread=thread::spawn(move || loop{
            let job=receiver.lock().unwrap().recv().unwrap();
            /*
            You can comment the code below to see how many connection is called
            The code handle multiple requests in the some connection without closing it
            unless Connection: close is specified
             */
            println!("Worker {id} got a job; executing.");
            job();
        });
       
    Self{id,thread}
    }
}