/*use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![0; 3]));

    let handles: Vec<_> = (0..3)
        .map(|i| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let mut data = data.lock().unwrap();
                data[i] += 1;
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", data.lock().unwrap());
}
    */

// Now same code but with a rwlock instead of a mutex
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(vec![0; 3]));

    let handles: Vec<_> = (0..3)
        .map(|i| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let mut data = data.write().unwrap();
                data[i] += 1;
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", data.read().unwrap());
}
