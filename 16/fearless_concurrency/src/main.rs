// fn sixteen_one () {
//     use std::thread;
//     use std::time::Duration;
//
//     thread::spawn(|| {
//         for i in 1..100 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//
//     for i in 1..50 {
//         println!("hi number {} from the main thread", i);
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// fn sixteen_two() {
//     use std::thread;
//     use std::time::Duration;
//
//     let handle = thread::spawn(|| {
//         for i in 1..100 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//
//     // handle.join().unwrap();
//
//     for i in 1..50 {
//         println!("hi number {} from the main thread", i);
//         thread::sleep(Duration::from_millis(1));
//     }
//     // handle.join().unwrap();
// }

// fn sixteen_three() {
//     use std::thread;
//
//     let v = vec![1, 2, 3,];
//
//     let handle = thread::spawn(|| {
//         println!("Heres a vector: {:?}", v);
//     });
//
//     handle.join().unwrap();
// }

// fn sixteen_four() {
//     use std::thread;
//
//     let v = vec![1, 2, 3,];
//
//     let handle = thread::spawn(|| {
//         println!("Heres a vector: {:?}", v);
//     });
//
//     drop(v);
//
//     handle.join().unwrap();
// }

// fn sixteen_five() {
//     use std::thread;
//
//     let v = vec![1, 2, 3,];
//
//     let handle = thread::spawn(move || {
//         println!("Heres a vector: {:?}", v);
//     });
//
//     // drop(v);
//
//     handle.join().unwrap();
// }

// fn sixteen_six() {
//     use std::sync::mpsc;
//
//     let (tx, rx) = mpsc::channel();
// }

// fn sixteen_seven() {
//     use std::thread;
//     use std::sync::mpsc;
//
//     let (tx, rx) = mpsc::channel();
//
//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//     });
// }
//
// fn sixteen_eight() {
//     use std::thread;
//     use std::sync::mpsc;
//
//     let (tx, rx) = mpsc::channel();
//
//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//     });
//
//     let received = rx.recv().unwrap();
//     println!("Got: {}", received);
// }

// fn sixteen_nine() {
//     use std::thread;
//     use std::sync::mpsc;
//
//     let (tx, rx) = mpsc::channel();
//
//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//         println!("val is {}", val);
//     });
//
//     let received = rx.recv().unwrap();
//     println!("Got: {}", received);
// }

// fn sixteen_ten() {
//     use std::thread;
//     use std::sync::mpsc;
//     use std::time::Duration;
//
//     let (tx, rx) = mpsc::channel();
//
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];
//
//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
//
//     for received in rx {
//         println!("Got : {}", received);
//     };
// }

// fn sixteen_eleven() {
//     use std::thread;
//     use std::sync::mpsc;
//     use std::time::Duration;
//
//     let (tx, rx) = mpsc::channel();
//
//     let tx1 = mpsc::Sender::clone(&tx);
//
//
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];
//
//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
//
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("more"),
//             String::from("messages"),
//             String::from("for"),
//             String::from("you"),
//         ];
//
//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
//
//     for received in rx {
//         println!("Got : {}", received);
//     };
// }

// fn sixteen_twelve() {
//     use std::sync::Mutex;
//
//     let m = Mutex::new(5);
//
//     {
//         let mut num = m.lock().unwrap();
//         *num = 6;
//     }
//
//     println!("m = {:?}", m);
// }

// fn sixteen_thirteen() {
//     use std::sync::Mutex;
//     use std::thread;
//
//     let counter = Mutex::new(0);
//     let mut handles = vec![];
//
//     let handle = thread::spawn(move || {
//         let mut num = counter.lock().unwrap();
//         *num += 1;
//     });
//     handles.push(handle);
//
//     let handle2 = thread::spawn(move || {
//         let mut num2 = counter.lock().unwrap();
//
//         *num2 += 1;
//     });
//
//     handles.push(handle2);
// }

// fn sixteen_fourteen() {
//     use std::rc::Rc;
//     use std::sync::Mutex;
//     use std::thread;
//
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];
//
//     for _ in 0..10 {
//         let counter = Rc::clone(&counter);
//         let handle = thread::spawn(move ||{
//             let mut num = counter.lock().unwrap();
//
//             *num +=1;
//         });
//         handles.push(handle);
//     }
//
//     for handle in handles {
//         handle.join().unwrap();
//     }
//
//     println!("Result: {}", *counter.lock().unwrap());
// }

fn sixteen_fifteen() {
    use std::sync::{Mutex, Arc};
    use std::thread;

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num +=1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn main() {
    // sixteen_one();
    // sixteen_two();
    // sixteen_three();
    // sixteen_four();
    // sixteen_five();
    // sixteen_six();
    // sixteen_seven();
    // sixteen_eight();
    // sixteen_nine();
    // sixteen_ten();
    // sixteen_eleven();
    // sixteen_twelve();
    // sixteen_thirteen();
    // sixteen_fourteen();
    sixteen_fifteen();
}
