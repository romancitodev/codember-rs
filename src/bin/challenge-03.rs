struct Challenge;

type Output = u32;

impl Challenge {
    fn solve(steps: &mut [i32]) -> Output {
        let mut index: i32 = 0;
        let mut counter = 0;
        while (0..steps.len()).contains(&(index as usize)) {
            let temp = steps[index as usize];
            steps[index as usize] += 1;
            counter += 1;
            index += temp;
        }
        counter
    }
}

fn main() {
    let lines = include_str!("../../data/challenge-03.txt").lines();

    let mut parsed_lines = lines.map(|line| {
        line.split(' ')
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });

    let mut last_line = parsed_lines.next_back().unwrap();
    let last = Challenge::solve(last_line.as_mut_slice());

    let result = parsed_lines
        .map(|mut line| Challenge::solve(line.as_mut_slice()))
        .sum::<u32>();

    println!("{}-{}", result + last, last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert!(Challenge::solve(&mut [1, 2, 4, 1, -2]) == 5);
        assert!(Challenge::solve(&mut [0, 1, 2, 3, -1]) == 6);
        assert!(Challenge::solve(&mut [1, -2, 5]) == 2);
        assert!(Challenge::solve(&mut [-20, -2, 5]) == 1);
    }
}
