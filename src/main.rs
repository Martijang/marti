use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{thread, time};

fn main(){
    let now = Instant::now();//starts to check a excution time(timer)
    let print_mutex = Arc::new(Mutex::new(()));

    let mut handle = vec![];//array
    let five_second = time::Duration::from_millis(5000);// five second(1000ms = 1 second)

    for i in 0..10  {
        let print_mutex = Arc::clone(&print_mutex);//cloning print_mutex to make sure the new owner of print_mutex
        let my_thread = thread::spawn(move ||{
            let _lock = print_mutex.lock().unwrap();//to make sure that variable i is printed in the right order
            println!("hello");// printing hellow

            thread::sleep(five_second);//waits for five second

            println!("Thread {} has completed", i);//printing the completed threads in the order
        });
        handle.push(my_thread);//pushing my_threads to handle array
    }

    for my_thread in  handle{
        my_thread.join().unwrap();//to make sure that my_threads works safely
    }

    let elapsed = now.elapsed();//stops timer and save it
    println!("Elapsed: {:.2?}", elapsed);//printing the excution time
    return;
}
