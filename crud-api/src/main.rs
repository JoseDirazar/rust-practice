#[macro_use]
extern crate rocket;
use std::io;
use rocket::tokio::task::spawn_blocking;
use rocket::tokio::time::{sleep, Duration};
//
#[get("/hello/<name>")]
fn index(name: &str) -> String {
    if name.len() != 0 {
        format!("Hello, {}", name)
    } else {
        format!("hello, world.")
    }
}
#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    let vec = spawn_blocking(|| std::fs::read("data.txt")).await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}


// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![index])
// }

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, delay, blocking_task])
        .launch()
        .await?;

    Ok(())
}
