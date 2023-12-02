use std::path::PathBuf;

use rocket::{get, routes, Build, Rocket};

/// # 🎄 Day -1: Get your winter boots on!
/// ## ⭐ Task 1: Everything is OK
/// Deploy a minimal working web app to your CCH23 Shuttle project.
///
/// The root endpoint `/` should respond with a `200 OK` status code to GET requests. Responding with
/// a "Hello, world!" string, like the starter templates do, is enough to accomplish this.
///
/// ### 💠 Example Input
/// ```sh
/// curl -I -X GET http://localhost:8000/
/// ```
///
/// ### 💠 Example Output
/// ```sh
/// HTTP/1.1 200 OK
/// ```
#[get("/")]
fn hello_world() -> &'static str {
    "Hello, world!"
}

/// # 🎄 Day -1: Get your winter boots on!
/// ## 🎁 Task 2: Fake error (0 bonus points)
/// For this bonus task, add an endpoint on `/-1/error` that responds with the status code `500 Internal
/// Server Error` to GET requests. The response body content does not matter.
///
/// ### 💠 Example Input
/// ```sh
/// curl -I -X GET http://localhost:8000/-1/error
/// ```
///
/// ### 💠 Example Output
/// ```sh
/// HTTP/1.1 500 Internal Server Error
/// ```
#[get("/error")]
fn error() -> &'static str {
    panic!()
}

/// # 🎄 Day 1: Packet "exclusive-cube" not found
/// ## ⭐ Task 1: Cube the bits
/// Santa needs your programming expertise to recalibrate the packet IDs in his packet management system.
///
/// Implement a GET endpoint `/1/<num1>/<num2>` that takes 2 integers in the path, `num1` and `num2`, and returns
/// the result of `(num1 XOR num2) POW 3`, where XOR is the exclusive OR operation, and POW is exponentiation.
///
/// ### 💠 Example Input
/// ```sh
/// curl http://localhost:8000/1/4/8
/// ```
///
/// ### 💠 Example Output
/// ```sh
/// 1728
/// ```
///
/// ## 🎁 Task 2: The sled ID system (100 bonus points)
/// After the packet IDs are calibrated and the packets are packed into sleds, Santa needs to calibrate the
/// sled ID.
///
/// The formula is very similar: All packet IDs (integers) are XOR'ed with each other, and then the result
/// is (again) raised to the power of 3. The catch is that there can be between 1 and 20 packets in a sled!
///
/// Adapt the endpoint from Task 1 so that it can also be used to calculate sled IDs.
///
/// ### 💠 Example Input
/// ```sh
/// curl http://localhost:8000/1/10
/// curl http://localhost:8000/1/4/5/8/10
/// ```
///
/// ### 💠 Example Output
/// ```sh
/// 1000
/// 27
/// ```
#[get("/<nums..>")]
fn cube_bits(nums: PathBuf) -> String {
    // convert the path buff into a vector of i32s
    let nums: Vec<i32> = nums
        .iter()
        .map(|x| x.to_str().unwrap().parse::<i32>().unwrap())
        .collect();

    i32::pow(nums.iter().fold(0, |acc, x| acc ^ x), 3).to_string()
}

fn ignite() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![hello_world])
        .mount("/-1", routes![error])
        .mount("/1", routes![cube_bits])
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = ignite();
    Ok(rocket.into())
}

#[cfg(test)]
mod unit_tests {
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

    #[test]
    fn test_cube_bits() {
        assert_eq!(cube_bits(PathBuf::from("4/8")), "1728");
        assert_eq!(cube_bits(PathBuf::from("10")), "1000");
        assert_eq!(cube_bits(PathBuf::from("4/5/8/10")), "27");
    }
}

#[cfg(test)]
mod integration_tests {
    use super::ignite;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    fn get_client() -> Client {
        let rocket = ignite();
        Client::tracked(rocket).unwrap()
    }

    #[test]
    fn test_hello_world() {
        let client = get_client();
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("Hello, world!".into()));
    }

    #[test]
    fn test_error() {
        let client = get_client();
        let response = client.get("/-1/error").dispatch();
        assert_eq!(response.status(), Status::InternalServerError);
    }

    #[test]
    fn test_cube_bits() {
        let client = get_client();
        let response = client.get("/1/4/8").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("1728".into()));

        let response = client.get("/1/10").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("1000".into()));

        let response = client.get("/1/4/5/8/10").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("27".into()));
    }
}
