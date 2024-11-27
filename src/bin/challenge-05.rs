codember_rs::import_file!("../../data/challenge-05.txt");

struct Challenge;

type Output = (usize, i32);

fn prime_sieve(limit: i32) -> Vec<i32> {
    let mut counter = vec![true; (limit as usize) + 1];

    let mut primes = Vec::new();
    let bound = (limit as f32).sqrt() as i32 + 1;
    for x in 2..bound {
        if counter[x as usize] {
            primes.push(x);
            let mut mul = x;
            while mul < limit {
                counter[mul as usize] = false;
                mul += x;
            }
        }
    }

    for x in bound..limit {
        if counter[x as usize] {
            primes.push(x)
        }
    }

    primes
}

impl Challenge {
    fn solve(input: &[i32]) -> Output {
        let primes = prime_sieve(*input.last().unwrap());
        let filtered = input
            .iter()
            .filter(|x| primes.contains(x))
            .filter(|v| {
                let n = v
                    .to_string()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .sum::<u32>();
                primes.contains(&(n as i32))
            })
            .collect::<Vec<_>>();
        (
            filtered.len(),
            **filtered.get(2).or(filtered.first()).unwrap(),
        )
    }
}

fn main() {
    let parsed = FILE
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let result = Challenge::solve(&parsed);
    println!("{result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Challenge::solve(&[11, 12, 13, 14]), (1, 11));
    }
}
