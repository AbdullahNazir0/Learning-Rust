use std::thread;
// use std::time::Duration;
use std::sync::mpsc;

fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         // println!("Hello from new thread {}", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    // handle.join().unwrap();
    // for i in 1..5 {
    //     // println!("Hello from main thread {}", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // let v = vec![1, 2, 3];
    // let handle = thread::spawn(move || {
    //     println!("Here's a vector: {:#?}", v);
    // });
    // handle.join().unwrap();

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val: String = String::from("hi");
        println!("Val in thread: {:?}", val);
        tx.send(val).unwrap();
        // println!("Val in thread after sending: {:?}", val);  // error, as it is moved.
    });
    let recieved = rx.recv().unwrap();
    println!("Got: {:?}", recieved);

}
