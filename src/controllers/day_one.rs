pub fn cube_bits(nums: &[i32]) -> i32 {
    let xor_result = nums.iter().fold(0, |acc, x| acc ^ x);
    i32::pow(xor_result, 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_bits() {
        assert_eq!(cube_bits(&vec![4, 8]), 1728);
        assert_eq!(cube_bits(&vec![10]), 1000);
        assert_eq!(cube_bits(&vec![4, 5, 8, 10]), 27);
    }
}
