
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_arg_parse() {
        assert_eq!(parse_config(&vec!["/path/to/main.rs".to_string(), "data/test.txt".to_string()]), "data/test.txt")
    }
}


pub fn parse_config(args: &[String]) -> &str {
    let filename = &args[1];
    filename
}
