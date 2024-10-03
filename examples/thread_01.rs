use std::thread;
use std::time::Duration;

fn main() {

    let thread_1 = thread::spawn(|| {
        for i in 1..=5 {
            println!("number {} from the spawned_1 thread!", i);
            thread::sleep(Duration::from_secs(2));
        }
    });


    let thread_2 = thread::spawn(|| {
        for i in 1..=5 {
            println!("number {} from the spawned_2 thread!", i);
            thread::sleep(Duration::from_secs(4));
        }
    });

    for i in 1..=5 {
        println!("number {} from the main thread!", i);
        thread::sleep(Duration::from_secs(8));
    }

    thread_1.join().unwrap();
    thread_2.join().unwrap();
}

// number 1 from the spawned_2 thread!
// number 1 from the spawned_1 thread!
// number 1 from the main thread!
// number 2 from the spawned_1 thread!
// number 2 from the spawned_2 thread!
// number 3 from the spawned_1 thread!
// number 4 from the spawned_1 thread!
// number 2 from the main thread!
// number 3 from the spawned_2 thread!
// number 5 from the spawned_1 thread!
// number 4 from the spawned_2 thread!
// number 3 from the main thread!
// number 5 from the spawned_2 thread!
// number 4 from the main thread!
// number 5 from the main thread!

