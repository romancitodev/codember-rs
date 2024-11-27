use std::collections::{HashMap, HashSet};

use lazy_regex::{regex, Captures};

codember_rs::import_file!("../../data/challenge-04.txt");

struct Challenge;

type Output = Vec<u8>;

impl Challenge {
    // Solved implementing DFS
    fn solve(input: &[[u8; 2]]) -> Output {
        let mut graph = HashMap::new();

        for &[a, b] in input {
            graph.entry(a).or_insert_with(Vec::new).push(b);
            graph.entry(b).or_insert_with(Vec::new).push(a);
        }

        let mut visited = HashSet::new();
        let mut result = Vec::new();

        fn search(
            node: u8,
            g: &HashMap<u8, Vec<u8>>,
            visitor: &mut HashSet<u8>,
            cache: &mut Vec<u8>,
        ) {
            if visitor.contains(&node) {
                return;
            }
            visitor.insert(node);
            cache.push(node);

            if let Some(neighbour) = g.get(&node) {
                for &n in neighbour {
                    search(n, g, visitor, cache);
                }
            }
        }

        for &node in graph.keys() {
            if !visited.contains(&node) {
                let mut cache = Vec::new();
                search(node, &graph, &mut visited, &mut cache);
                if cache.len() < 3 {
                    result.extend(cache);
                }
            }
        }
        result.sort();
        result
    }
}

fn process_str(input: &str) -> Vec<[u8; 2]> {
    let re = regex!(r"(?:\[(\d{1,}),\s*(\d{1,})*\])");
    let parser = |c: Captures<'_>| [c[1].parse::<u8>(), c[2].parse::<u8>()].map(Result::unwrap);
    re.captures_iter(input).map(parser).collect::<Vec<_>>()
}

fn main() {
    let input = process_str(FILE);
    let input = input.as_slice();
    let result = Challenge::solve(input);
    let result = result
        .iter()
        .fold(String::new(), |acc, c| format!("{acc},{c}"));
    println!("{result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_case() {
        let input = process_str("[[1,2],[2,3],[4,5]]");
        let input = input.as_slice();
        assert_eq!(input, [[1, 2], [2, 3], [4, 5]]);
        assert_eq!(Challenge::solve(input), [4, 5]);
    }

    #[test]
    fn second_case() {
        let input = process_str("[[1,2],[2,3],[3,4]]");
        let input = input.as_slice();
        assert_eq!(Challenge::solve(input), []);
    }

    #[test]
    fn third_case() {
        let input = process_str("[[4,6],[7,9],[10,12],[12,16]]");
        let input = input.as_slice();
        assert_eq!(Challenge::solve(input), [4, 6, 7, 9]);
    }
}
