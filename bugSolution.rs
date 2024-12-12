use std::sync::{Arc, Mutex};

fn main() {
    let x = Arc::new(Mutex::new(5));
    let y = x.clone();
    let z = x.clone();

    let _t1 = std::thread::spawn(move || {
        let mut x_locked = y.lock().unwrap();
        *x_locked = 10;
    });
    let _t2 = std::thread::spawn(move || {
        let mut x_locked = z.lock().unwrap();
        *x_locked = 12; 
    });
    
    let _t1 = std::thread::spawn(move || {
        let mut x_locked = y.lock().unwrap();
        *x_locked = 10;
    });
    let _t2 = std::thread::spawn(move || {
        let mut x_locked = z.lock().unwrap();
        *x_locked = 12; 
    });
    
    _t1.join().unwrap();
    _t2.join().unwrap();

    println!("x = {}", *x.lock().unwrap());
}