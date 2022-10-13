#[macro_use] extern crate rocket;
use rocket::tokio::time::{sleep, Duration};
use rocket::fs::FileServer;
use rocket::fs::relative;


#[get("/hi")]
fn index() -> &'static str {
    "Hi!"
}

#[get("/world")]
fn index_world() -> &'static str {
    "world!"
}

#[get("/delay/<seconds>")]      // <> denotes a dynamic path
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, index_world, delay ])
        .mount("/public", FileServer::from(relative!("/src")))          // serve whatever is on src folder. E.g. http://localhost:8000/public/main.rs
}