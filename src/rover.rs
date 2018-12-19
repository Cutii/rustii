pub fn hello_world(name: String) -> String {
    if name == String::from("") {
        String::from("hello world")
    } else {
        String::from("hello ") + &name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_empty_then_return_hello_world() {
        let h1 = hello_world(String::from(""));
        assert_eq!(String::from("hello world"), h1);
    }

    #[test]
    fn given_cutii_then_return_hello_cutii() {
        let h2 = hello_world(String::from("cutii"));
        assert_eq!(String::from("hello cutii"), h2);
    }
}
