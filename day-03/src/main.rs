use std::collections::HashMap;

type BluePrint = HashMap<usize, Vec<char>>;

struct NumberAddress {
    number: usize,
    x: Vec<usize>,
    y: usize,
}

fn main() {
    // let input = include_str!("../input");
    let input = include_str!("../example");
    let data = input.lines();
    let mut blueprint: BluePrint = HashMap::new();

    // Arrange data into a hashmap
    for (i, line) in data.enumerate() {
        let linechar: Vec<char> = line.chars().collect();
        blueprint.insert(i, linechar.clone());
        // println!("{:?}, {:?}", i, linechar);
    }
    // for (i, map) in blueprint {
    //     println!("{:?}, {:?}", i, map);
    // }

    // Sweep for number addresses
    let mut numbers: Vec<NumberAddress> = vec![];
    for (y, line) in blueprint.clone() {
        let mut temp_number: String = String::new();
        let mut temp_addr: Vec<usize> = vec![];
        for (x, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                // println!("x {:?}", x);
                // println!("y {:?}", ey);
                temp_number.push(*c);
                temp_addr.push(x);
            } else if temp_number.contains(|c: char| c.is_ascii_digit()) {
                let number_address = NumberAddress {
                    number: temp_number.parse().unwrap(),
                    x: temp_addr.clone(),
                    y,
                };
                temp_number.clear();
                temp_addr.clear();
                numbers.push(number_address);
            }
        }
    }

    // Check numbers for actual numbers
    let mut verified_numbers: Vec<NumberAddress> = vec![];
    for (i, num) in numbers.iter().enumerate() {
        println!("Num: {:?}, x: {:?}, y: {:?}", num.number, num.x, num.y);

        let mut sec_end = num.x.last().unwrap().to_owned();
        if sec_end < blueprint[&num.y].len() {
            sec_end += 1;
        }

        let mut sec_first = num.x.first().unwrap().to_owned();
        sec_first = sec_first.saturating_sub(1);

        let line_next = if num.y < blueprint.len() {
            num.y + 1
        } else {
            num.y
        };
        println!("Line Next {}", line_next);
        println!("Line Past {}", num.y.saturating_sub(1));

        let mut block: Vec<char> = vec![];
        for y in [num.y.saturating_sub(1), num.y, line_next] {
            let bp = blueprint[&y][sec_first..=sec_end].iter();
            println!("Section: {:?}", bp);
            // block.append(&mut blueprint[&y][sec_first..=sec_end]);
        }
        println!("Block: {:?}", block);

        println!("Section end: {:?}", sec_end);
        println!("Section first: {:?}", sec_first);
    }
}
