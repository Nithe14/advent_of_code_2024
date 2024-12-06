use std::collections::HashMap;
use std::fs::read_to_string;
use utils::get_input_path;

fn main() {
    let input_path = get_input_path("d5");
    let input = read_to_string(input_path).unwrap_or_else(|err| {
        eprintln!("Cannot read input file!: {err}");
        std::process::exit(1);
    });

    let (rules, updates) = split_input(&input).unwrap_or_else(|| {
        eprintln!("Cannot parse input string!");
        std::process::exit(1);
    });

    let map = create_map(rules);
    let valids = get_values(updates, &map, true);
    let result = sum_middles(&valids);

    println!("Correctly-ordered middles sum: {}", result);
    let mut invalids = get_values(updates, &map, false);
    invalids.iter_mut().for_each(|line| make_valid(line, &map));
    let result = sum_middles(&invalids);
    println!("Incorrectly-ordered updates fixed middles sum: {}", result);
}

fn create_map(list: &str) -> HashMap<i32, Vec<i32>> {
    let mut map = HashMap::new();

    for line in list.lines() {
        let nums: Vec<i32> = line.split('|').filter_map(|c| c.parse().ok()).collect();
        if nums.len() != 2 {
            break;
        }
        map.entry(nums[0])
            .and_modify(|v: &mut Vec<i32>| v.push(nums[1]))
            .or_insert(vec![nums[1]]);
    }

    map
}

fn is_valid(line: &Vec<i32>, map: &HashMap<i32, Vec<i32>>) -> bool {
    for (i, num) in line.iter().enumerate() {
        if let Some(invlaid_before) = map.get(num) {
            for value in invlaid_before {
                if line[..i].iter().any(|n| n == value) {
                    return false;
                }
            }
        }
    }
    true
}

fn get_values(input: &str, map: &HashMap<i32, Vec<i32>>, return_valid: bool) -> Vec<Vec<i32>> {
    let mut valid = Vec::new();
    for line in input.lines() {
        let l: Vec<i32> = line.split(',').filter_map(|c| c.parse().ok()).collect();
        if is_valid(&l, &map) == return_valid {
            valid.push(l);
        }
    }

    valid
}

fn sum_middles(vec: &Vec<Vec<i32>>) -> i32 {
    vec.iter().map(|v| v[v.len() / 2]).sum()
}

fn split_input(input: &str) -> Option<(&str, &str)> {
    let tmp: Vec<&str> = input.split("\n\n").map(|s| s.trim()).collect();
    Some((tmp.get(0)?, tmp.get(1)?))
}

fn make_valid(line: &mut Vec<i32>, map: &HashMap<i32, Vec<i32>>) {
    let mut i = 0;
    while i < line.len() {
        let num = line[i];

        if let Some(invalid_before) = map.get(&num) {
            for &value in invalid_before.iter() {
                if let Some(index) = line[..i].iter().position(|&n| n == value) {
                    line.swap(i, index);
                    i = 0;
                    break;
                }
            }
        }
        i += 1;
    }
}

#[test]
fn examples() {
    let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    let (rules, updates) = split_input(input).unwrap();

    let map = create_map(rules);
    let valids = get_values(&updates, &map, true);

    assert_eq!(
        vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13]
        ],
        valids
    );

    assert_eq!(143, sum_middles(&valids));

    let mut invalid_vec = vec![97, 13, 75, 29, 47];
    make_valid(&mut invalid_vec, &map);

    assert_eq!(vec![97, 75, 47, 29, 13], invalid_vec);

    let mut invalids = get_values(&updates, &map, false);
    invalids.iter_mut().for_each(|line| make_valid(line, &map));

    assert_eq!(123, sum_middles(&invalids));
}
