use crate::controllers::day_negative_one;
use rocket::{get, routes, Route};

#[get("/")]
fn hello_world() -> &'static str {
    day_negative_one::hello_world()
}

#[get("/-1/error")]
fn error() -> &'static str {
    day_negative_one::error()
}

pub fn routes() -> Vec<Route> {
    routes![hello_world, error]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        assert_eq!(hello_world(), "Hello, world!");
    }

    #[test]
    #[should_panic]
    fn test_error() {
        error();
    }
}