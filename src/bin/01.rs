advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u16> {
    return input
        .lines()
        .filter_map(|line| {
            let mut digits = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as u16);
            let first = digits.next()?;
            let last = digits.last()?;
            Some(first * 10 + last)
        })
        .sum::<u16>()
        .into();
}

pub fn part_two(input: &str) -> Option<u32> {
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    return input
        .lines()
        .filter_map(|line| {
            let mut p_line = line.to_string();
            for (index, &word) in nums.iter().enumerate() {
                p_line = p_line.replace(
                    word,
                    &format!("{}{}{}", word, (index + 1).to_string(), word),
                );
            }
            let digits: Vec<char> = p_line.chars().filter(|c| c.is_digit(10)).collect();
            if digits.is_empty() {
                return None;
            } else {
                let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
                    .parse::<u32>()
                    .unwrap();
                return Some(number);
            }
        })
        .sum::<u32>()
        .into();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
