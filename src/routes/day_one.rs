use rocket::{get, routes, Route};
use std::path::PathBuf;

use crate::controllers::day_one;

#[get("/<nums..>")]
fn cube_bits(nums: PathBuf) -> String {
    // convert the path buff into a vector of i32s
    let nums: Vec<i32> = nums
        .iter()
        .map(|x| x.to_str().unwrap().parse::<i32>().unwrap())
        .collect();
    day_one::cube_bits(&nums).to_string()
}

pub fn routes() -> Vec<Route> {
    routes![cube_bits]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_bits() {
        assert_eq!(cube_bits(PathBuf::from("4/8")), "1728");
        assert_eq!(cube_bits(PathBuf::from("10")), "1000");
        assert_eq!(cube_bits(PathBuf::from("4/5/8/10")), "27");
    }
}
