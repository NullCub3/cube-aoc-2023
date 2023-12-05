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
        let mut temp_number: String = String::new();
        let mut temp_addr: Vec<usize> = vec![];
        for (i, x) in y.iter().enumerate() {
            if x.is_ascii_digit() {
                println!("x {:?}", x);
                println!("y {:?}", ey);
                temp_number.push(*x);
                temp_addr.push(i);
            } else if temp_number.contains(|c: char| c.is_ascii_digit()) {
                println!("Numbertemp: {}", &temp_number);
                number_address.number = temp_number.parse().unwrap();
                number_address.x = temp_addr.clone();
                number_address.y = ey;
                println!("{:?}", number_address.x);
                temp_number.clear();
                temp_addr.clear();
                //numbers.push(number_address);
            }
        }
    }
}
