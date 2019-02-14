
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {

        let input_data = "+1\n-2\n+3".to_string();
        let output_data = vec!(1, -2, 3);

        assert_eq!(parse(&input_data), output_data);
    }
}


pub fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}
