use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use utils::get_input_path;

fn main() {
    let input_path = get_input_path("d1");
    let (left, right) = parse_input(&input_path).unwrap_or_else(|err| {
        eprintln!("Cannot parse input file: {err}");
        std::process::exit(1);
    });

    let result = total_distance(&left, &right);
    println!("Total distance: {}", result);
    let result = similarity_score(&left, &right);
    println!("Similarity score: {}", result);
}

fn parse_input(path: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn Error>> {
    let file_string = read_to_string(path)?;
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in file_string.lines() {
        let mut tuple = line.split_whitespace();
        let left_value = tuple.next().ok_or("Missing left value!")?.parse::<u32>()?;
        let right_value = tuple.next().ok_or("Missing right value!")?.parse::<u32>()?;
        left.push(left_value);
        right.push(right_value);
    }

    Ok((left, right))
}

fn total_distance(left: &[u32], right: &[u32]) -> u32 {
    let mut left = Vec::from(left);
    let mut right = Vec::from(right);
    left.sort();
    right.sort();

    left.iter().zip(right).map(|(l, r)| r.abs_diff(*l)).sum()
}

fn similarity_score(left: &[u32], right: &[u32]) -> u32 {
    let mut right_stats = HashMap::new();

    for &id in right {
        *right_stats.entry(id).or_insert(0) += 1;
    }

    left.iter()
        .map(|id| right_stats.get(id).unwrap_or(&0) * id)
        .sum()
}

#[test]
fn example1() {
    let input = (&[3, 4, 2, 1, 3, 3], &[4, 3, 5, 3, 9, 3]);
    let result = total_distance(input.0, input.1);

    assert_eq!(11, result);
}

#[test]
fn example2() {
    let input = (&[3, 4, 2, 1, 3, 3], &[4, 3, 5, 3, 9, 3]);
    let result = similarity_score(input.0, input.1);

    assert_eq!(31, result);
}
