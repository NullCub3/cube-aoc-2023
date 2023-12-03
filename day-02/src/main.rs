struct Game {
    id: usize,
    max_red: usize,
    max_green: usize,
    max_blue: usize,
}

// The possible numbers of cubes are:
// 12 red cubes, 13 green cubes, and 14 blue cubes

fn main() {
    // let input = include_str!("../input");
    let input = include_str!("../example");
    let lines = input.lines();

    for (i, line) in lines.enumerate() {
        println!("{:?}", line);
    }
}
