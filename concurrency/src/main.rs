use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..20 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // using join handles
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the JOINED spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // join() will blcok the current running thread (main)
    handle.join().unwrap();

    let v = vec![1, 2, 3];

    // use move to take the ownership. reference is not allowed.
    let handle1 = thread::spawn(move || {
        println!("Here's a MOVED vector: {v:?}");
    });

    handle1.join().unwrap();

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let msg = String::from("I'm a squirrel!");
        tx.send(msg).unwrap();
    });

    // let receiveed_msg = rx.recv().unwrap();
    // println!("I got your message '{}'", receiveed_msg);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
