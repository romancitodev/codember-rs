struct Challenge;

impl Challenge {
    fn solve(combination: &str, pattern: &str) -> String {
        let mut digits = combination
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        let mut index = 0;
        let len = digits.len();

        for action in pattern.chars() {
            match action {
                'R' => index = (index + 1) % len,
                'L' => index = (index + len - 1) % len,
                'U' => digits[index] = (digits[index] + 1) % 10,
                'D' => digits[index] = (digits[index] + 9) % 10,
                _ => unreachable!("You mustn't be here"),
            }
        }
        digits.iter().map(|d| d.to_string()).collect()
    }
}

fn main() {
    let result = Challenge::solve("528934712834", "URDURUDRUDLLLLUUDDUDUDUDLLRRRR");
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let combination = "000";
        let pattern = "URURD";
        assert!(Challenge::solve(combination, pattern) == *"119");
    }

    #[test]
    fn test_second_solution() {
        let combination = "1111";
        let pattern = "UUURUUU";
        assert!(Challenge::solve(combination, pattern) == *"4411");
    }
}
