/*
* $ time ./target/release/day06 inputs/d6
*
* Number of distinct position: [redacted]
* Number of different positions could be choosen for obstructions: [redacted]
* ________________________________________________________
* Executed in  784.29 millis    fish           external
* usr time  702.16 millis  305.00 micros  701.86 millis
* sys time   78.73 millis   84.00 micros   78.65 millis
*
*/
mod navigation;
use std::collections::HashSet;
use std::fs::read_to_string;

use navigation::{Direction, Position};
use utils::get_input_path;

fn main() {
    let input_path = get_input_path("d6");
    let input = read_to_string(input_path).unwrap_or_else(|err| {
        eprintln!("Cannot read input file!: {err}");
        std::process::exit(1);
    });
    let position = find_begining(&input).unwrap_or_else(|| {
        eprintln!("Cannot find begining guard position!");
        std::process::exit(1);
    });
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let result = guard_patrol(&grid, &position, &Direction::Up).unwrap();
    let (distinct_positions, guard_positions) = get_guard_route(&result);
    println!("Number of distinct position: {}", distinct_positions);

    let result = count_obstructions(&mut grid, &guard_positions, &position, &Direction::Up);
    println!("Number of different positions could be choosen for obstructions: {result}");
}

fn get_guard_route(guard_route: &[Vec<i32>]) -> (usize, Vec<(usize, usize)>) {
    let mut guard_positions = Vec::new();
    let number_of_distinct_positions = guard_route
        .iter()
        .enumerate()
        .map(|(x, row)| {
            let value = &mut guard_positions;
            row.iter()
                .enumerate()
                .filter(|&(_, &value)| value == 1)
                .map(move |(y, _)| {
                    value.push((x, y));
                    1
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    (number_of_distinct_positions, guard_positions)
}

/*
 * This function should be rewritten to find all occurrences of '<', '>', '^', 'v'
 * and return:
 * - Ok(Some(Position, Direction)) if exactly one position is found,
 * - Ok(None) if no position is found,
 * or
 * - Err(...) if more than one possible position is found,
 *   but handling this case is out of scope for this task.
 */
fn find_begining(input: &str) -> Option<Position> {
    input
        .lines()
        .enumerate()
        .find_map(|(i, line)| line.find("^").map(|found| Position::new((i, found))))
}

// returns None if the patrol is looped
fn guard_patrol(
    grid: &[Vec<char>],
    begining: &Position,
    starting_direction: &Direction,
) -> Option<Vec<Vec<i32>>> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut position = *begining;
    let mut direction = *starting_direction;
    let mut result = vec![vec![0; cols]; rows];
    let mut visited: HashSet<(Position, Direction)> = HashSet::new();

    while !position.is_out_of_scope(rows, cols) {
        if !visited.insert((position, direction)) {
            return None;
        }

        let x = position.x();
        let y = position.y();

        if grid[x][y] == '#' {
            position.jump(&direction.oposite()); //go back
            direction.rotate();
            continue;
        }

        result[x][y] = 1;
        position.jump(&direction);
    }

    Some(result)
}

fn count_obstructions(
    grid: &mut [Vec<char>],
    positions: &[(usize, usize)],
    begining: &Position,
    starting_direction: &Direction,
) -> u32 {
    let mut counter = 0;

    for &(row, col) in positions {
        let curr = grid[row][col];
        if curr == '#' || curr == '^' {
            continue;
        }

        grid[row][col] = '#';

        if guard_patrol(&grid, &begining, &starting_direction).is_none() {
            counter += 1;
        }
        grid[row][col] = '.';
    }

    counter
}

#[test]
fn example1() {
    let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    let position = find_begining(&input).unwrap();

    assert_eq!(Position::new((6, 4)), position);

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let result = guard_patrol(&grid, &position, &Direction::Up).unwrap();
    let (test, _) = get_guard_route(&result);

    assert_eq!(41, test);
}

#[test]
fn example2() {
    let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#.O^.....
........#.
#.........
......#..."#;
    let position = find_begining(input).unwrap();
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == 'O' { '#' } else { c })
                .collect()
        })
        .collect();
    let result = guard_patrol(&grid, &position, &Direction::Up);

    assert_eq!(None, result);
}

#[test]
fn example3() {
    let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    let position = find_begining(&input).unwrap();

    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let guard_route = guard_patrol(&grid, &position, &Direction::Up).unwrap();
    println!("{:?}", guard_route);

    let (_, guard_positions) = get_guard_route(&guard_route);
    let result = count_obstructions(&mut grid, &guard_positions, &position, &Direction::Up);

    assert_eq!(6, result);
}
