use std::{collections::HashMap, usize};

type BluePrint = HashMap<usize, Vec<char>>;

#[derive(Clone, Debug)]
struct NumberAddress {
    number: usize,
    x: Vec<usize>,
    y: usize,
}

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    let data = input.lines();
    let mut blueprint: BluePrint = HashMap::new();

    // Arrange data into a hashmap
    for (i, line) in data.enumerate() {
        let linechar: Vec<char> = line.chars().collect();
        blueprint.insert(i, linechar.clone());
    }

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
    for (_i, num) in numbers.iter().enumerate() {
        // println!("Num: {:?}, x: {:?}, y: {:?}", num.number, num.x, num.y);

        let sec_end = num
            .x
            .last()
            .unwrap()
            .to_owned()
            .clamp(0, blueprint[&num.y].len() - 1)
            .saturating_add(1);
        //println!("sec_end: {:?}", sec_end);

        let sec_first = num.x.first().unwrap().to_owned().saturating_sub(1);

        let line_y = num.y.clamp(0, blueprint.len() - 2);
        // println!("bp len: {:?}", blueprint.len());
        // println!("line: {:?}", line_y);
        // println!("line_next: {:?}", line_y.saturating_add(1));

        'linecheck: for y in [line_y.saturating_sub(1), line_y, line_y.saturating_add(1)] {
            //println!("y: {:?}", y);
            for c in blueprint[&y][sec_first..=sec_end].iter() {
                if c.is_ascii_digit() || c == &'.' {
                } else {
                    verified_numbers.push(num.clone());
                    break 'linecheck;
                }
            }
        }
    }
    // println!("verified_numbers: {:?}", verified_numbers);
    let mut bp_sum = 0;
    for vn in verified_numbers {
        // println!("Verified Number: {:?}", vn.y);
        bp_sum += vn.number;
    }

    println!("bp_sum: {:?}", bp_sum);
}
