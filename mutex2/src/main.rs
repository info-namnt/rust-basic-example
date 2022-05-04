use parking_lot::{Mutex, ReentrantMutex};
use std::{sync::Arc, rc::Rc, thread};
use std::time::Duration;
use backoff::ExponentialBackoff;

type ArcAccount = Arc<Mutex<Account>>;

struct Account {
    balance: usize
}

fn transfer(from: ArcAccount, to: ArcAccount, amount: usize) {
    let op = || {
        if let Some(mut from) = from.try_lock() {
            if let Some(mut to) = to.try_lock() {
                from.balance -= amount;
                to.balance += amount;
                return Ok(());
            }
        }
        Err(0) ?
    };

    // Tối ưu hóa thời gian sleep cho thread 
    let backoff = ExponentialBackoff::default();
    backoff::retry(backoff, op);
}
fn main() {

    let transaction_1 = thread::spawn(move || {
        transfer(a, b, 100);
    });

    let transaction_2 = thread::spawn(move || {
        transfer(b, a, 300);
    });
}
