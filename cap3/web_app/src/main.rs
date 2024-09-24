use std;

use actix_web;

async fn greet(req: actix_web::HttpRequest) -> impl actix_web::Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    return format!("Hello {name}");
}

fn do_something(number: i8) -> i8 {
    println!("number {number} is running");
    let two_seconds = core::time::Duration::from_secs(2);
    std::thread::sleep(two_seconds);
    return 2;
}

async fn sync_learning(req: actix_web::HttpRequest) -> impl actix_web::Responder {
    let now = std::time::Instant::now();

    let first: i8 = req.match_info().get("first").unwrap().parse().unwrap_or(1);
    let second: i8 = req.match_info().get("second").unwrap().parse().unwrap_or(2);
    let third: i8 = req.match_info().get("third").unwrap().parse().unwrap_or(3);

    let one = do_something(first);
    let two = do_something(second);
    let three = do_something(third);

    return format!(
        "time elapsed {time_elapsed:?} \n\n result {total}",
        time_elapsed = now.elapsed(),
        total = one + two + three
    );
}

async fn async_learning(req: actix_web::HttpRequest) -> impl actix_web::Responder {
    let now = std::time::Instant::now();

    let first: i8 = req.match_info().get("first").unwrap().parse().unwrap_or(1);
    let second: i8 = req.match_info().get("second").unwrap().parse().unwrap_or(2);
    let third: i8 = req.match_info().get("third").unwrap().parse().unwrap_or(3);

    let thread_one = std::thread::spawn(move || do_something(first));
    let thread_two = std::thread::spawn(move || do_something(second));
    let thread_three = std::thread::spawn(move || do_something(third));

    let result_one = thread_one.join().unwrap_or(1);
    let result_two = thread_two.join().unwrap_or(2);
    let result_three = thread_three.join().unwrap_or(3);

    return format!(
        "time elapsed {time_elapsed:?} \n\n result {total}",
        time_elapsed = now.elapsed(),
        total = result_one + result_two + result_three
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .route("/", actix_web::web::get().to(greet))
            .route(
                "/say/hello",
                actix_web::web::get().to(|| async { "Hello Again!" }),
            )
            .route(
                "/sync/{first}/{second}/{third}",
                actix_web::web::get().to(sync_learning),
            )
            .route(
                "/async/{first}/{second}/{third}",
                actix_web::web::get().to(async_learning),
            )
            .route("/{name}", actix_web::web::get().to(greet))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
