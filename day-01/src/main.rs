fn main() {
    // Get File
    let input = include_str!("../input");
    // Split file into lines
    let lines = input.lines();

    // Variable to add to
    let mut result = 0;

    // Main process, iterating over the lines in the input
    for l in lines {
        // Found off of stack overflow, uses regex I think to get all the numeric values from the text.
        let t: Vec<_> = l.chars().filter(|c| c.is_digit(10)).collect();

        // Get first and last values
        let mut hello = String::new();
        hello.push(*t.first().unwrap());
        hello.push(*t.last().unwrap());

        // Adding to running total
        result = result + hello.parse::<i32>().unwrap();
    }

    println!("The total value is: {}", result);
}
