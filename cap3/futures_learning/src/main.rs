use std;

use futures;

fn main() {
    let now = std::time::Instant::now();
    // let future_one = do_something(1);
    // let future_two = async {
    //     return do_something(2).await;
    // };
    // let future_three = async {
    //     let result_one = do_something(2);
    //     let result_two = do_something(3);
    //     let results = futures::join!(result_one, result_two);
    //     return results.0 + results.1;
    // };
    // let outcome = futures::executor::block_on(future_three);

    let async_outcome = async {
        let mut futures_vec = Vec::new();
        let future_four = do_something(4);
        let future_five = do_something(5);

        futures_vec.push(future_four);
        futures_vec.push(future_five);

        let handles = futures_vec
            .into_iter()
            .map(async_std::task::spawn)
            .collect::<Vec<_>>();

        let results = futures::future::join_all(handles).await;

        return results.into_iter().sum::<i8>();
    };

    let result = futures::executor::block_on(async_outcome);

    println!("time elapsed {:?}", now.elapsed());
    println!("Here is the outcome: {}", result);
}

async fn do_something(number: i8) -> i8 {
    println!("number {number} is running");
    let two_seconds = std::time::Duration::from_secs(2);
    std::thread::sleep(two_seconds);
    return 2;
}
