use std::path::PathBuf;

use itertools::Itertools;
use rocket::{
    get, post, routes,
    serde::{json::Json, Deserialize, Serialize},
    Build, Rocket,
};

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

/// # 🎄 Day 4: What do you call a serialized reindeer? Serdeer!
/// Under the soft glow of the Northern Lights, Santa's reindeer
/// are training for the big night. But, oh deer! The reindeer's
/// stats have been serialized into an unknown format after a playful
/// elf accidentally spilled hot cocoa on the central computer. The
/// data needs to be deserialized so the reindeer team can be assembled
/// based on their combined strength.
///
/// ## ⭐ Task 1: Reindeer cheer
/// The task is to create a POST endpoint /4/strength that calculates the
/// combined strength of a group of reindeer, so that Santa knows if they
/// can pull his sled through the skies.
///
/// The input to the endpoint is a JSON array containing information about
/// each reindeer. Each reindeer is represented as an object with two
/// attributes: "name" (string) and "strength" (integer). Collect the strength
/// of each reindeer and respond with the sum.
///
/// ### 💠 Example Input
/// ```sh
/// curl -X POST http://localhost:8000/4/strength \
///      -H 'Content-Type: application/json' \
///      -d '[
///             { "name": "Dasher", "strength": 5 },
///             { "name": "Dancer", "strength": 6 },
///             { "name": "Prancer", "strength": 4 },
///             { "name": "Vixen", "strength": 7 }
///          ]'
/// ```
///
/// ### 💠 Example Output
/// ```sh
/// 22
/// ```
#[post("/strength", data = "<reindeer>")]
fn reindeer_strength(reindeer: Json<Vec<Reindeer>>) -> String {
    reindeer
        .iter()
        .fold(0, |acc, x| acc + x.strength)
        .to_string()
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Reindeer {
    name: String,
    strength: i32,
}

/// # 🎄 Day 4: What do you call a serialized reindeer? Serdeer!
/// ## 🎁 Task 2: Cursed candy eating contest (150 bonus points)
/// This time, there is some more data for each reindeer (see example).
/// The endpoint is called /4/contest, because the reindeer are now going
/// to compare the attributes of the reindeer and present a summary of the
/// winners.
///
/// There is at least one reindeer participating in the contest, and there
/// is never a tie for first place.
///
/// For some reason, one of the field names in the input seems to still be
/// a bit corrupted from the incident. Guess we just have to deal with it anyways.
///
/// The output should be a JSON object containing a summary of the winners
/// (see example).
///
/// ### 💠 Example Input
/// ```sh
/// curl -X POST http://localhost:8000/4/contest \
///      -H 'Content-Type: application/json' \
///      -d '[
///               {
///                 "name": "Dasher",
///                 "strength": 5,
///                 "speed": 50.4,
///                 "height": 80,
///                 "antler_width": 36,
///                 "snow_magic_power": 9001,
///                 "favorite_food": "hay",
///                 "cAnD13s_3ATeN-yesT3rdAy": 2
///               },
///               {
///                 "name": "Dancer",
///                 "strength": 6,
///                 "speed": 48.2,
///                 "height": 65,
///                 "antler_width": 37,
///                 "snow_magic_power": 4004,
///                 "favorite_food": "grass",
///                 "cAnD13s_3ATeN-yesT3rdAy": 5
///               }
///         ]'
/// ```
///
/// ### 💠 Example Output
/// Note: JSON output examples are sometimes formatted. Output from your endpoint does not need to be formatted. The output is parsed and checked as a value.
/// ```sh
/// {
///   "fastest": "Speeding past the finish line with a strength of 5 is Dasher",
///   "tallest": "Dasher is standing tall with his 36 cm wide antlers",
///   "magician": "Dasher could blast you away with a snow magic power of 9001",
///   "consumer": "Dancer ate lots of candies, but also some grass"
/// }
/// ```
#[post("/contest", data = "<reindeer>")]
fn candy_eating_contest(reindeer: Json<Vec<ReindeerExtended>>) -> Json<ContestResult> {
    let reindeer = reindeer.into_inner();
    let fastest = reindeer
        .iter()
        .max_by(|a, b| a.speed.partial_cmp(&b.speed).unwrap())
        .unwrap();
    let tallest = reindeer
        .iter()
        .max_by(|a, b| a.height.cmp(&b.height))
        .unwrap();
    let magician = reindeer
        .iter()
        .max_by(|a, b| a.snow_magic_power.cmp(&b.snow_magic_power))
        .unwrap();
    let consumer = reindeer
        .iter()
        .max_by(|a, b| a.candies_eaten_yesterday.cmp(&b.candies_eaten_yesterday))
        .unwrap();

    Json(ContestResult {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.reindeer.strength, fastest.reindeer.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.reindeer.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.reindeer.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.reindeer.name, consumer.favorite_food
        ),
    })
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct ReindeerExtended {
    #[serde(flatten)]
    reindeer: Reindeer,
    speed: f32,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct ContestResult {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

/// # 🎄 Day 6: Elf on a shelf
/// It's that time of year when the elves hide on shelves to watch
/// over the children of the world, reporting back to Santa on who's
/// been naughty or nice. However, this year's reports have been mixed
/// up with the rest of the letters to Santa, and the word "elf" is
/// hidden throughout a mountain of text.
///
/// ## ⭐ Task 1: Never count on an elf
/// Elves are notorious for their hide-and-seek skills, and now they've
/// hidden themselves in strings of text!
///
/// Create an endpoint /6 that takes a POST request with a raw string as
/// input and count how many times the substring "elf" appears.
///
/// The output should be a JSON object containing the count of the string "elf".
///
/// ### 💠 Examples
/// ```sh
/// curl -X POST http://localhost:8000/6 \
///      -H 'Content-Type: text/plain' \
///      -d 'The mischievous elf peeked out from behind the toy workshop,
///          and another elf joined in the festive dance.
///          Look, there is also an elf on that shelf!'
///
/// {"elf":4}
/// ```
/// ## 🎁 Task 2: Shelf under an elf? (200 bonus points)
/// Add two fields to the response that counts:
///
/// The number of occurrences of the string "elf on a shelf" in a field with
/// the same name.
/// The number of shelves that don't have an elf on it. That is, the number
/// of strings "shelf" that are not preceded by the string "elf on a ". Put
/// this count in the field "shelf with no elf on it".
///
/// ### 💠 Example
/// ```sh
/// curl -X POST http://localhost:8000/6 \
///      -H 'Content-Type: text/plain' \
///      -d 'there is an elf on a shelf on an elf.
///          there is also another shelf in Belfast.'
///
/// {"elf":5,"elf on a shelf":1,"shelf with no elf on it":1}
/// ```
#[post("/", data = "<text>")]
fn elf_on_a_shelf(text: String) -> Json<ElvesOnShelves> {
    let elf = text.matches("elf").count();
    let elf_on_a_shelf = text.matches("elf on a shelf").count();
    let shelf_with_no_elf_on_it = text
        .matches("shelf")
        .zip_longest(text.matches("elf on a shelf"))
        .filter(|item| item.is_left())
        .count();

    Json(ElvesOnShelves {
        elf,
        elf_on_a_shelf,
        shelf_with_no_elf_on_it,
    })
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct ElvesOnShelves {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: usize,
}

fn ignite() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![hello_world])
        .mount("/-1", routes![error])
        .mount("/1", routes![cube_bits])
        .mount("/4", routes![reindeer_strength, candy_eating_contest])
        .mount("/6", routes![elf_on_a_shelf])
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = ignite();
    Ok(rocket.into())
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    use rocket::serde::json;

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

    #[test]
    fn test_reindeer_strength() {
        let reindeer = Json::<Vec<Reindeer>>(vec![
            Reindeer {
                name: String::from("Dasher"),
                strength: 5,
            },
            Reindeer {
                name: String::from("Dancer"),
                strength: 6,
            },
            Reindeer {
                name: String::from("Prancer"),
                strength: 4,
            },
            Reindeer {
                name: String::from("Vixen"),
                strength: 7,
            },
        ]);
        assert_eq!(reindeer_strength(reindeer), "22");
    }

    #[test]
    fn test_candy_eating_contest() {
        let reindeer = Json(vec![
            ReindeerExtended {
                reindeer: Reindeer {
                    name: String::from("Dasher"),
                    strength: 5,
                },
                speed: 50.4,
                height: 80,
                antler_width: 36,
                snow_magic_power: 9001,
                favorite_food: String::from("hay"),
                candies_eaten_yesterday: 2,
            },
            ReindeerExtended {
                reindeer: Reindeer {
                    name: String::from("Dancer"),
                    strength: 6,
                },
                speed: 48.2,
                height: 65,
                antler_width: 37,
                snow_magic_power: 4004,
                favorite_food: String::from("grass"),
                candies_eaten_yesterday: 5,
            },
        ]);
        let expected_output = ContestResult {
            fastest: String::from("Speeding past the finish line with a strength of 5 is Dasher"),
            tallest: String::from("Dasher is standing tall with his 36 cm wide antlers"),
            magician: String::from("Dasher could blast you away with a snow magic power of 9001"),
            consumer: String::from("Dancer ate lots of candies, but also some grass"),
        };

        let response = candy_eating_contest(reindeer);
        assert_eq!(
            json::to_pretty_string(&response.0).unwrap(),
            json::to_pretty_string(&expected_output).unwrap()
        );
    }

    #[test]
    fn test_elf_on_a_shelf() {
        let result1 = elf_on_a_shelf(String::from(
            "The mischievous elf peeked out from behind the toy workshop,
            and another elf joined in the festive dance.
            Look, there is also an elf on that shelf!",
        ))
        .0;

        assert_eq!(result1.elf, 4);
        assert_eq!(result1.elf_on_a_shelf, 0);
        assert_eq!(result1.shelf_with_no_elf_on_it, 1);

        let result2 = elf_on_a_shelf(String::from(
            "there is an elf on a shelf on an elf.
            there is also another shelf in Belfast.",
        ))
        .0;

        assert_eq!(result2.elf, 5);
        assert_eq!(result2.elf_on_a_shelf, 1);
        assert_eq!(result2.shelf_with_no_elf_on_it, 1);
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::ElvesOnShelves;

    use super::{ignite, ContestResult, Reindeer};

    use indoc::indoc;
    use rocket::{
        http::Status,
        local::blocking::Client,
        serde::json::{self, json, Value},
    };

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

    #[test]
    fn test_reindeer_strength() {
        let client = get_client();
        let response = client
            .post("/4/strength")
            .json(&json!(vec![
                Reindeer {
                    name: String::from("Dasher"),
                    strength: 5,
                },
                Reindeer {
                    name: String::from("Dancer"),
                    strength: 6,
                },
                Reindeer {
                    name: String::from("Prancer"),
                    strength: 4,
                },
                Reindeer {
                    name: String::from("Vixen"),
                    strength: 7,
                },
            ]))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("22".into()));
    }

    #[test]
    fn test_candy_eating_contest() {
        let client = get_client();
        let reindeer = indoc! {r#"
            [
                {
                    "name": "Dasher",
                    "strength": 5,
                    "speed": 50.4,
                    "height": 80,
                    "antler_width": 36,
                    "snow_magic_power": 9001,
                    "favorite_food": "hay",
                    "cAnD13s_3ATeN-yesT3rdAy": 2
                },
                {
                    "name": "Dancer",
                    "strength": 6,
                    "speed": 48.2,
                    "height": 65,
                    "antler_width": 37,
                    "snow_magic_power": 4004,
                    "favorite_food": "grass",
                    "cAnD13s_3ATeN-yesT3rdAy": 5
                }
            ]
        "#};
        let response = client.post("/4/contest").body(reindeer).dispatch();
        let expected_output = ContestResult {
            fastest: String::from("Speeding past the finish line with a strength of 5 is Dasher"),
            tallest: String::from("Dasher is standing tall with his 36 cm wide antlers"),
            magician: String::from("Dasher could blast you away with a snow magic power of 9001"),
            consumer: String::from("Dancer ate lots of candies, but also some grass"),
        };
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            json::from_str::<Value>(&response.into_string().unwrap()).unwrap(),
            json::from_str::<Value>(&json!(expected_output).to_string()).unwrap()
        );
    }

    #[test]
    fn test_elf_on_a_shelf() {
        let client = get_client();
        let response = client
            .post("/6")
            .body(indoc! {r#"
                The mischievous elf peeked out from behind the toy workshop,
                and another elf joined in the festive dance.
                Look, there is also an elf on that shelf!
            "#})
            .dispatch();

        let expected_output = ElvesOnShelves {
            elf: 4,
            elf_on_a_shelf: 0,
            shelf_with_no_elf_on_it: 1,
        };
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            json::from_str::<Value>(&response.into_string().unwrap()).unwrap(),
            json::from_str::<Value>(&json!(expected_output).to_string()).unwrap()
        );

        let response = client
            .post("/6")
            .body(indoc! {r#"
                there is an elf on a shelf on an elf.
                there is also another shelf in Belfast.
            "#})
            .dispatch();

        let expected_output = ElvesOnShelves {
            elf: 5,
            elf_on_a_shelf: 1,
            shelf_with_no_elf_on_it: 1,
        };
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            json::from_str::<Value>(&response.into_string().unwrap()).unwrap(),
            json::from_str::<Value>(&json!(expected_output).to_string()).unwrap()
        );
    }
}
