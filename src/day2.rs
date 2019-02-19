#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec!("abcdef".to_string(), "bababc".to_string(),
        "abbcde".to_string(), "abcccd".to_string(), "aabcdd".to_string(),
        "abcdee".to_string(), "ababab".to_string());
        let output = 12;
        assert_eq!(part_1(&input), output)

    }
}


pub fn part_1(input: &Vec<String>) -> i32 {

    let mut two_count = 0;
    let mut three_count = 0;

    for item in input {

        let mut two_match = false;
        let mut three_match = false;

        for character in item.chars() {
            let match_count = item.matches(character).count();
            if match_count == 2 {two_match = true;}
            if match_count == 3 {three_match = true;}
        }
        if two_match {
            two_count += 1;
        }
        if three_match {
            three_count += 1;
        }
    }

    two_count * three_count
}
