#[cfg(test)]
mod tests {
    use super::*;

    fn is_hello<T: AsRef<str>>(s: T) {
        assert_eq!("hello", s.as_ref());
    }

    #[test]
    fn test_asref() {
        let s = "hello";
        is_hello(s);

        let s = "hello".to_string();
        is_hello(s);
    }
}
