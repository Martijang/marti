use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{thread, time};

fn main(){
    let now = Instant::now();
    let print_mutex = Arc::new(Mutex::new(()));

    let mut handle = vec![];
    let five_second = time::Duration::from_millis(5000);

    for i in 0..10  {
        let print_mutex = Arc::clone(&print_mutex);
        let my_thread = thread::spawn(move ||{
            let _lock = print_mutex.lock().unwrap();
            println!("hello");

            thread::sleep(five_second);

            println!("Thread {} has completed", i);
        });
        handle.push(my_thread);
    }

    for my_thread in  handle{
        my_thread.join().unwrap();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    return;
}