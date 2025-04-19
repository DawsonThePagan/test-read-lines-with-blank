use read_lines_with_blank::*;

fn main() {
    let lines = read_lines_with_blank("input.txt").unwrap();

    for line in lines {
        println!("{}", line);
    }

    let lines = read_lines_with_blank_from_str("line1\n\nline2\n\nline3").unwrap();
    for line in lines {
        println!("{}", line);
    }
}
