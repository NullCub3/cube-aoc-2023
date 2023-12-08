type BluePrint = Vec<Vec<char>>;

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
    // let mut blueprint: BluePrint = HashMap::new();
    let mut blueprint: BluePrint = Vec::new();

    // Arrange data into a hashmap
    for (_i, line) in data.enumerate() {
        let linechar: Vec<char> = line.chars().collect();
        blueprint.push(linechar);
    }

    // Sweep for number addresses
    let mut numbers: Vec<NumberAddress> = vec![];
    for (y, line) in blueprint.clone().iter().enumerate() {
        let mut temp_number: String = String::new();
        let mut temp_addr: Vec<usize> = vec![];
        for (x, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                // println!("x {:?}", x);
                // println!("y {:?}", ey);
                temp_number.push(*c);
                temp_addr.push(x);
            } else if !temp_number.is_empty() {
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
        if !temp_number.is_empty() {
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

    // Check numbers for actual numbers
    let mut verified_numbers: Vec<NumberAddress> = vec![];
    for (_i, num) in numbers.iter().enumerate() {
        // println!("Num: {:?}, x: {:?}, y: {:?}", num.number, num.x, num.y);

        let sec_end = num
            .x
            .last()
            .unwrap()
            .to_owned()
            .saturating_add(1)
            .min(blueprint[num.y].len() - 1);

        let sec_first = num.x.first().unwrap().to_owned().saturating_sub(1);

        // println!("bp len: {:?}", blueprint.len());
        // println!("line: {:?}", line_y);
        // println!("line_next: {:?}", line_y.saturating_add(1));
        // println!("{:?}", num.number);
        // println!("-- x: {:?} y: {:?} num: {:?} --", num.x, num.y, num.number);

        'linecheck: for y in [
            num.y.saturating_sub(1),
            num.y,
            num.y.saturating_add(1).min(blueprint.len() - 1),
        ] {
            println!("{:?}", &blueprint[y][sec_first..=sec_end]);
            for c in blueprint[y][sec_first..=sec_end].iter() {
                if c.is_ascii_digit() || c == &'.' {
                } else {
                    verified_numbers.push(num.clone());
                    println!("[ Found Symbol ]");
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
