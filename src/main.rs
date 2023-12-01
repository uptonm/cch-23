mod controllers;
mod routes;

use routes::{day_negative_one, day_one};

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", day_negative_one::routes())
        .mount("/1", day_one::routes());

    Ok(rocket.into())
}
