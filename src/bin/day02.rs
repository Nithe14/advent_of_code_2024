use std::error::Error;
use std::fs::read_to_string;
use utils::get_input_path;

fn main() {
    let path = get_input_path("d2");
    let input = read_to_string(path).unwrap_or_else(|err| {
        eprintln!("Cannot read file: {err}");
        std::process::exit(1);
    });
    let result = safe_reports_count(&input, false).unwrap_or_else(|err| {
        eprintln!("Cannot calculate: {err}");
        std::process::exit(1);
    });

    println!("Number of safe reports: {}", result);

    let result = safe_reports_count(&input, true).unwrap_or_else(|err| {
        eprintln!("Cannot calculate: {err}");
        std::process::exit(1);
    });
    println!("Number of safe reports with Problem Dampener: {}", result);
}

/*
 * Returns:
 * None -> the vector is valid,
 * Some(index) -> the vector is invalid, index is the first candidate for removal
 */
fn invalid_index(line_vec: &[i32], index: Option<usize>) -> Option<usize> {
    let mut sign: Option<i32> = None;
    let mut invalid_index: usize = 0;
    let line_vec =
        line_vec
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| if Some(i) == index { None } else { Some(v) });

    let mut iter = line_vec.peekable();
    while let Some(num) = iter.next() {
        invalid_index += 1; //iterating from 1 not 0

        if let Some(&next) = iter.peek() {
            let diff = num - next;
            if (1..=3).contains(&diff.abs()) //  differ level 1..=3
    // all increasing/all decreasing or first iteration
    && (sign == Some(diff.signum()) || sign.is_none())
            {
                sign = Some(diff.signum());
            } else {
                return Some(if invalid_index > 1 {
                    invalid_index - 2 //previous
                } else {
                    invalid_index - 1 //current
                });
            }
        }
    }

    None
}

fn safe_reports_count(input: &str, problem_dampener: bool) -> Result<u32, Box<dyn Error>> {
    let mut counter = 0;
    for line in input.lines() {
        let line_vec: Vec<i32> = line
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        if invalid_index(&line_vec, None).is_none() {
            counter += 1;
            continue;
        }
        if let Some(index) = invalid_index(&line_vec, None) {
            if problem_dampener
                && (index..line_vec.len()).any(|i| invalid_index(&line_vec, Some(i)).is_none())
            {
                counter += 1;
            }
        }
    }

    Ok(counter)
}

#[test]
fn example1() {
    let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";

    let result = safe_reports_count(input, false);

    assert_eq!(2, result.unwrap());
}

#[test]
fn example2() {
    let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";

    let result = safe_reports_count(input, true);

    assert_eq!(4, result.unwrap());
}
