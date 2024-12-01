fn main() {
    println!(
        "{}", 
        wrap_text("Advent of Code is an Advent calendar of small programming puzzles for a variety of skill levels that can be solved in any programming language you like. People use them as interview prep, company training, university coursework, practice problems, a speed contest, or to challenge each other.", 60
    ));
    println!(
        "\n{}",
        wrap_text("Visit for more: https://adventofcode.com/2024/about", 60)
    );
}

fn wrap_text(text: &str, max_width: usize) -> String {
    let mut wrapped = String::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > max_width {
            wrapped.push_str(&current_line);
            wrapped.push('\n');
            current_line.clear();
        }
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }

    wrapped.push_str(&current_line);
    wrapped
}
