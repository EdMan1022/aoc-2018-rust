#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_1() {
        let input = "abcdef";
        let output = "";
        assert_eq!(part_1(input.to_string()), output)

    }

    #[test]
    fn test_part_1_2() {
        let input = "bababc";
        let output = "";
        assert_eq!(part_1(input.to_string()), output)

    }
    #[test]
    fn test_part_1_3() {
        let input = "abbcde";
        let output = "";
        assert_eq!(part_1(input.to_string()), output)
    }
    #[test]
    fn test_part_1_4() {
        let input = "abcccd";
        let output = "";
        assert_eq!(part_1(input.to_string()), output)
    }
    #[test]
    fn test_part_1_5() {
        let input = "aabcdd";
        let output = "";
        assert_eq!(part_1(input.to_string()), output)
    }

    #[test]
    fn test_part_1_6() {
        let input = "abcdee";
        let output = "";
        assert_eq!(part_1(input.to_string()), output)
        
    }

    #[test]
    fn test_part_1_7() {
        let input = "ababab";
        let output = "";
        assert_eq!(part_1(input.to_string()), output)        
    }
}

pub fn part_1(input: String) -> String {
    "output".to_string()
}
