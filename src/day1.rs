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

/// Takes a vector of integers and returns the sum
/// 
/// # Arguments
/// 
/// * `data` - a Vector of integers that can be added
/// 
/// # Example
/// ```
/// use day1::part1;
/// let input = vec!(1, 2, 3)
/// let output = part1(input);
/// ```
pub fn part1(data: &Vec<i32>) -> i32 {

    
    let mut sum = 0;

    for i in data.iter() {
        sum += i;
    }
    sum
}