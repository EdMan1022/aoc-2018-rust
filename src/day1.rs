#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec!(1, 2, 3);
        let output = 6;

        assert_eq!(part1(&input), output)
    }

    #[test]
    fn test_part_1_negative() {
        let input = vec!(1, -2, 3);
        let output = 2;

        assert_eq!(part1(&input), output)
    }

}

pub fn part1(data: &Vec<i32>) -> i32 {
    32
}