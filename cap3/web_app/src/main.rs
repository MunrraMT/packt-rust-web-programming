use std::{thread, time};

fn main() {
    let now = time::Instant::now();

    let thread_one = thread::spawn(|| do_something(1));
    let thread_two = thread::spawn(|| do_something(2));
    let thread_three = thread::spawn(|| do_something(3));

    let result_one = thread_one.join();
    let result_two = thread_two.join();
    let result_three = thread_three.join();

    println!("time elapsed {:?}", now.elapsed());
    println!(
        "result {}",
        result_one.unwrap() + result_two.unwrap() + result_three.unwrap()
    );
}

fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);

    2
}
