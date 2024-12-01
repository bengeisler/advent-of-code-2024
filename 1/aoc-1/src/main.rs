use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {}

fn calculate_total_distance(list: Vec<(i32, i32)>) -> i32 {
    let mut left = list.iter().map(|(l, _)| *l).collect::<Vec<i32>>();
    let mut right = list.iter().map(|(_, r)| *r).collect::<Vec<i32>>();
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn calculate_similarity_score(list: Vec<(i32, i32)>) -> i32 {
    let right = list.iter().map(|(_, r)| *r).collect::<Vec<i32>>();
    list.iter()
        .map(|(l, _)| *l)
        .map(|l| l * (right.iter().filter(|&r| *r == l).count() as i32))
        .sum()
}

fn read_list_from_file(filename: &str) -> io::Result<Vec<(i32, i32)>> {
    let file = File::open(Path::new(filename))?;
    let reader = io::BufReader::new(file);
    let list_of_tuples = reader
        .lines()
        .filter_map(|line| line.ok().and_then(|line| parse_line_to_tuple(&line)))
        .collect();
    Ok(list_of_tuples)
}

fn parse_line_to_tuple(line: &str) -> Option<(i32, i32)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 2 {
        return None;
    }
    if let (Ok(a), Ok(b)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
        return Some((a, b));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_total_distance_from_example_list() {
        let list = read_list_from_file("../example_list.txt").unwrap();
        let result = calculate_total_distance(list);
        assert_eq!(result, 11);
    }
    #[test]
    fn calculate_total_distance_from_puzzle_list() {
        let list = read_list_from_file("../puzzle_list.txt").unwrap();
        let result = calculate_total_distance(list);
        assert_eq!(result, 2580760);
    }

    #[test]
    fn calculate_similarity_score_from_example_list() {
        let list = read_list_from_file("../example_list.txt").unwrap();
        let result = calculate_similarity_score(list);
        assert_eq!(result, 31);
    }

    #[test]
    fn calculate_similarity_score_from_puzzle_list() {
        let list = read_list_from_file("../puzzle_list.txt").unwrap();
        let result = calculate_similarity_score(list);
        assert_eq!(result, 25358365);
    }
}
