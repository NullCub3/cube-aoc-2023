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
    for (i, y) in blueprint {
        let ey = i;
        let mut number_address = NumberAddress {
            number: 0,
            x: vec![],
            y: i,
        };
        let mut numbertemp: String = String::new();
        for (i, x) in y.iter().enumerate() {
            if x.is_ascii_digit() {
                println!("x {:?}", x);
                println!("y {:?}", ey);
                numbertemp.push(*x);
                number_address.x.push(i);
            } else if numbertemp.contains(|c: char| c.is_ascii_digit()) {
                println!("Numbertemp: {}", &numbertemp);
                println!("{:?}", number_address.x);
                number_address.number = numbertemp.parse().unwrap();
                number_address.y = ey;
                numbertemp.clear();
                //numbers.push(number_address);
            }
        }
    }
}
