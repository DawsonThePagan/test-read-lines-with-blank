use read_lines_with_blank::read_lines_with_blank;

fn main() {
    let lines = read_lines_with_blank("input.txt").unwrap();

    for line in lines {
        println!("{}", line);
    }
}
