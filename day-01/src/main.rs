use std::collections::HashMap;

fn main() {
    // Get File
    // let input = include_str!("../input");
    let input = include_str!("../input-trunc");

    // Split file into lines
    let lines = input.lines();

    // Variable to add to
    let mut result = 0;

    // Defining Hashmap of words
    type WordMap = HashMap<&'static str, char>;
    let word_map: WordMap = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    // println!("{:?}", word_map);

    // Main process, iterating over the lines in the input
    for l in lines {
        let mut line_nums: Vec<char> = vec![];
        let mut chars_string: String = String::new();
        type WordPosition = HashMap<String, usize>;
        let mut line_word_positions: WordPosition = HashMap::new();
        type NumPosition = HashMap<char, usize>;
        let mut line_num_position: NumPosition = HashMap::new();

        for (i, c) in l.chars().enumerate() {
            if c.is_ascii_digit() {
                line_num_position.insert(c, i);
                println!("NumPositions: {:?}", line_num_position);

                println!("Line Chars Cleared: {:?}", chars_string);
                chars_string.clear();
            } else {
                for (key, value) in word_map.iter() {
                    if chars_string.contains(&key.to_string())
                        && !line_word_positions.contains_key(&key.to_string())
                    {
                        println!("Found {} and added to words", key);
                        line_word_positions.insert(key.to_string(), i);
                        println!("WordPositons: {:?}", line_word_positions);

                        // line_nums.push(*value);
                    } else if chars_string.contains(&key.to_string()) {
                        println!("Found {} but was already in words", key);
                    } else {
                        // print!("{} ", key);
                    }
                }
                // println!("<- Hashes Checked");
                chars_string.push(c);
                println!("Current Chars: {:?}", chars_string);
            }
        }

        println!("Final Nums: {:?}", line_nums);
        println!("Final Chars: {:?}", chars_string);
        println!("Final WordPositons: {:?}", line_word_positions);
        println!("Final Num Positions: {:?}", line_num_position);

        // Get first and last values
        let mut hello = String::new();
        // hello.push(*line_nums.first().unwrap());
        // hello.push(*line_nums.last().unwrap());

        // Adding to running total
        // result += hello.parse::<i32>().unwrap();
    }

    // println!("The total value is: {}", result);
}
