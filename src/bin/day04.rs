/*
 *
 *
 * THE RESULTS ARE VALID BUT THIS IS STILL A WORK IN PROGRESS
 *
 *
 */

use std::fs::read_to_string;
use utils::get_input_path;

fn main() {
    let input_path = get_input_path("d4");
    let input = read_to_string(input_path).unwrap_or_else(|err| {
        eprintln!("Cannot parse input file!: {err}");
        std::process::exit(1);
    });

    let x_directions: &[i32; 8] = &[-1, -1, -1, 0, 0, 1, 1, 1];
    let y_directions: &[i32; 8] = &[-1, 0, 1, -1, 1, -1, 0, 1];
    let result = search_for_word(&input, "XMAS", x_directions, y_directions);
    println!("XMAS appears {} times!", result);

    let result = find_x_pattern(&input);
    println!("X-MAS appears {} times!", result);
}

fn search_for_word(input: &str, word: &str, x_directions: &[i32], y_directions: &[i32]) -> u32 {
    let mut counter = 0;
    let word: Vec<char> = word.chars().collect();
    let grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] != word[0] {
                continue;
            }

            for (x, y) in x_directions.iter().zip(y_directions) {
                let mut w = 1;
                let mut curr_x = row as i32 + x;
                let mut curr_y = col as i32 + y;
                for _ in 1..word.len() {
                    if curr_x < 0 || curr_y < 0 || curr_x >= rows as i32 || curr_y >= cols as i32 {
                        break;
                    }
                    if grid[curr_x as usize][curr_y as usize] != word[w] {
                        break;
                    }
                    curr_x += x;
                    curr_y += y;
                    w += 1;
                }

                if w == word.len() {
                    counter += 1;
                }
            }
        }
    }

    counter
}

fn is_in_bounds(x: i32, y: i32, rows: usize, cols: usize) -> bool {
    x >= 0 && y >= 0 && (x as usize) < rows && (y as usize) < cols
}

fn find_x_pattern(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let directions = [
        (-1, -1), //left-up
        (1, -1),  //left-down
    ];
    let mut counter = 0;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] != 'A' {
                continue;
            }
            let mut valid_neighbours = 0;
            for (dx, dy) in directions {
                let n1 = (row as i32 + dx, col as i32 + dy);
                let n2 = (row as i32 - dx, col as i32 - dy); //opposite direction

                if !is_in_bounds(n1.0, n1.1, rows, cols) || !is_in_bounds(n2.0, n2.1, rows, cols) {
                    break;
                }

                let neighbour1 = grid[n1.0 as usize][n1.1 as usize];
                let neighbour2 = grid[n2.0 as usize][n2.1 as usize];

                if (neighbour1 == 'M' && neighbour2 == 'S')
                    || (neighbour1 == 'S' && neighbour2 == 'M')
                {
                    valid_neighbours += 2;
                    continue;
                }
            }

            if valid_neighbours == 4 {
                counter += 1;
            }
        }
    }
    counter
}

#[test]
fn example1() {
    let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

    let x_directions: &[i32; 8] = &[-1, -1, -1, 0, 0, 1, 1, 1];
    let y_directions: &[i32; 8] = &[-1, 0, 1, -1, 1, -1, 0, 1];
    assert_eq!(
        18,
        search_for_word(input, "XMAS", x_directions, y_directions)
    );
}

#[test]
fn example2() {
    let input = r#".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."#;

    let result = find_x_pattern(input);
    assert_eq!(9, result);
}
