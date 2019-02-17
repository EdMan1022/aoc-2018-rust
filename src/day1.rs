use std::collections::HashSet;

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

    #[test]
    fn test_part2_1() {
        let input = vec!(1, -1);
        let output = 0;
        assert_eq!(part2(input), output)
    }
    #[test]
    fn test_part2_2() {
        let input = vec!(3, 3, 4, -2, -4);
        let output = 10;
        assert_eq!(part2(input), output)
    }
    #[test]
    fn test_part2_3() {
        let input = vec!(-6, 3, 8, 5, -6);
        let output = 5;
        assert_eq!(part2(input), output)
    }
    #[test]
    fn test_part2_4() {
        let input = vec!(7, 7, -2, -7, -4);
        let output = 14;
        assert_eq!(part2(input), output)
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

struct Frequencies {
    changes: Vec<i32>,
    updated: HashSet<i32>,
    current: i32
}

impl Frequencies {
    fn new(changes: Vec<i32>, updated: HashSet<i32>, current: i32) -> Frequencies {
        Frequencies{ 
            changes: changes,
            updated: updated,
            current: current
        }
    }
}

/// Finds the first repeat sum of adajecent elements in a vector
/// 
/// # Arguments
/// 
/// * `data` - a Vector of integers that can be added
/// 
/// # Example
/// ```
/// use day1::part2;
/// let input = vec!(3, 3, 4, -2, -4)
/// let output = part2(input);
/// assert_eq!(output, 10)
/// ```
pub fn part2(data: Vec<i32>) -> i32 {

    let updated = HashSet::new();
    let current = 0;

    let mut freqs = Frequencies::new(data, updated, current);

    freqs.updated.insert(current);

    loop {
        for change in &freqs.changes {
            let new = freqs.current + change;

            if freqs.updated.contains(&new) {
                return new;
            }

            freqs.updated.insert(new);
            freqs.current = new;

        }
    }
}
