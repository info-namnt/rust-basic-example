use std::{thread, time::Duration};

fn main() {
    let iterations = 10;
    // ---------------------------------------------------Example 1-------------------
    let t1 = thread::spawn(move || {
        for i in 1..=100 {
            println!("A: {}", i)
        }
    });

    let t2 = thread::spawn(move || {
        for i in 1..=10 {
            println!("B: {}", i)
        }
    });

    t1.join();
    t2.join();
    // ---------------------------------------------------Example 2-------------------
    // let t1 = thread::spawn(move || {
    //     thread::sleep(Duration::from_secs(1));
    //     50
    // });

    // println!("waiting for data");

    // match t1.join() {
    //     Ok(value) => println!("value: {}", value),
    //     Err(e)  => println!("error: {:?}", e)
        
    // }
}


