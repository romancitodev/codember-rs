use std::{fs::File, io::Write};

struct Challenge;

type Output = bool;

impl Challenge {
    fn solve(password: &str) -> Output {
        let mut letter_already = false;
        let mut last_char = 'a';
        let mut last_number = 0;
        let len = password.chars().take_while(|f| match f {
            c @ 'a'..='z' => {
                letter_already = true;
                if last_char > *c {
                    false
                } else {
                    last_char = *c;
                    true
                }
            }
            n @ '0'..='9' => {
                let n = n.to_digit(10).unwrap();
                if last_number > n || letter_already {
                    false
                } else {
                    last_number = n;
                    true
                }
            }
            _ => false,
        });
        len.count() == password.len()
    }
}

fn main() {
    let (valid, invalid): (Vec<_>, Vec<_>) = include_str!("../../data/challenge-02.txt")
        .lines()
        .partition(|f| Challenge::solve(f));

    let mut output = File::create("output/challenge-02.log").unwrap();

    write!(
        &mut output,
        "valid:\n{}\n====================\ninvalid:\n{}",
        valid.join("\n"),
        invalid.join("\n")
    )
    .unwrap();

    println!(
        "{valid}true{invalid}false",
        valid = valid.len(),
        invalid = invalid.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert!(Challenge::solve("1234"));
        assert!(!Challenge::solve("a123"));
        assert!(Challenge::solve("123abc"));
        assert!(Challenge::solve("1234"));
        assert!(!Challenge::solve("dbce"));
    }
}
