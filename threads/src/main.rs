use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    basic_demo();
    shared_state();
    message_passing();
}

fn shared_state() {
    // wrap the mutex in a atomic reference-counted smart pointer (shared ownership)
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![]; // initialize empty vector

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // increases reference count
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // Mutex provides interior mutability!
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Counter: {}", *counter.lock().unwrap());

}

fn message_passing() {
    // multiple-producer single-consumer
    let (tx, rx) = mpsc::channel();
    // tx: transmitter
    // rx: receiver
    let tx1 = tx.clone(); // create a second transmitter

    thread::spawn(move || {
        let vals = [
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap(); // send takes ownership of val
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = [
            String::from("even"),
            String::from("more"),
            String::from("messages"),
            String::from("from a second thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap(); // send takes ownership of val
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        // not calling recv explicitly and instead treating rx as an iterator
        println!("Got {received}");
    }
    // channel is closed when iteration ends

    // let received = rx.recv().unwrap(); //rx.recv blocks execution
}

fn basic_demo() {
    // join blocks the main thread from running until thread repented by handle terminates.
    let v = vec![1, 2, 3];

    // move forces closure to take ownership of the values it is using
    let handle = thread::spawn(move || {
        println!("Here is vector {v:?}");
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("Hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
