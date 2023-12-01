pub fn hello_world() -> &'static str {
    "Hello, world!"
}

pub fn error() -> &'static str {
    panic!()
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
