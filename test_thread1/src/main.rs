use std::thread;
use std::time::Duration;

fn main() {
    let mut handle_vector = vec!();
    for index in 0..5{
        // * creating a thread and passing a closure as code
        let handle = thread::spawn(move || {
            for i in 0..10 {
                println!("\tI'm the Thread {index}! Count is {i}");
                thread::sleep(Duration::from_millis(300));
            }
        });
        handle_vector.push(handle);
    }
    for i in 0..10 {
        println!("I'm the main Thread! Count is {i}");
        thread::sleep(Duration::from_secs(1));
    }


    for handle in handle_vector{
        handle.join().unwrap();
    }
}


