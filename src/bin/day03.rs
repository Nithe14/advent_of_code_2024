use regex::Regex;
use std::fs::read_to_string;
use std::sync::LazyLock;
use utils::get_input_path;

static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

fn main() {
    let input_path = get_input_path("d3");
    let input = read_to_string(input_path).unwrap_or_else(|err| {
        eprintln!("Cannot parse input file: {err}");
        std::process::exit(1);
    });
    let result = sum_multipies(&input);
    println!("Corrupted memory multipies sum: {}", result);

    let result = sum_multipies_skipping(&input);
    println!(
        "Corrupted memory multipies sum with do() and  don't() functions: {}",
        result
    );
}

fn sum_multipies(input: &str) -> i32 {
    RE.captures_iter(input)
        .filter_map(|groups| {
            //get(0) returns the entire match, not a regex group
            let x: i32 = groups.get(1)?.as_str().parse().ok()?;
            let y: i32 = groups.get(2)?.as_str().parse().ok()?;
            Some(x * y)
        })
        .sum()
}

fn sum_multipies_skipping(input: &str) -> i32 {
    let mut sum = 0;
    let mut remaining = input;

    while let Some(disable) = remaining.find("don't()") {
        sum += sum_multipies(&remaining[..disable]);

        if let Some(enable) = remaining[disable..].find("do()") {
            remaining = &remaining[disable + enable..];
        } else {
            return sum;
        }
    }
    sum += sum_multipies(remaining);
    sum
}

#[test]
fn example1() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let result = sum_multipies(input);
    assert_eq!(161, result);
}

#[test]
fn example2() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let result = sum_multipies_skipping(input);
    assert_eq!(48, result);
}
