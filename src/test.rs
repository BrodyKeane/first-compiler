#[cfg(test)]
mod tests {
    use crate::test_file::test_file;

    #[test]
    fn test_compiler() {
        let path = "tests/fib.lax";
        test_file(path)
    }
}
