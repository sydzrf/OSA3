use ndarray::arr2;
//use rayon::prelude::*;
use std::time::{Instant};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
static NO_THREADS: i32 = 3;

fn main(){
    // Creating endpoints of the channel
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();




    







    for thread_no in 0..NO_THREADS {
        //Cloning the sender
        let thread_tx = tx.clone();
        // Sending threads via the channel
        thread::spawn(move || {
            let start = Instant::now();
            let a = arr2(&[[1, 1, 1],
                           [1, 1, 1],
                           [1, 0, 1]]);
            let b = arr2(&[[1, 0, 0],
                           [0, 1, 0],
                           [0, 0, 1]]);
                           
            println!("{}", a.dot(&b));
            let duration = start.elapsed();     
            println!("Time elapsed in expensive_function() is: {:?}", duration);

            thread_tx.send(thread_no).unwrap();
            println!("thread {} finished", thread_no);
        });
    }
    // collecting all the threads 
    let mut thread_holder = Vec::with_capacity(NO_THREADS as usize);
    for _i in 0..NO_THREADS {
        // Get the message from channel 
        thread_holder.push(rx.recv().unwrap());
    }
    //Print the execution order
    println!("{:?}", thread_holder);
}
