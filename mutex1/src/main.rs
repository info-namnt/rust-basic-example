// Mutex
// Arc - Atomic reference counted
// Lib parking_lot faster then stander

use std::{sync::Arc, thread};

use parking_lot::Mutex;

struct Number(usize);
fn main() {
    let number = Number(0);

    let number_thread = Arc::new(Mutex::new(number));

    let number_thread1 = Arc::clone(&number_thread);

    let number_thread2 = number_thread.clone();

    let thread_1 = thread::spawn(move || {
        let mut number_plus = number_thread1.lock();
        number_plus.0 += 20;
    });


    let thread_2 = thread::spawn(move || {
        let mut number_plus = number_thread2.lock();
        number_plus.0 += 32;
    });

    thread_1.join().and_then(|_| thread_2.join());

    println!("{}", number_thread.lock().0);
}
