fn main() {
    let input = include_str!("../input");
    // let input = "1abc2";
    let lines = input.lines();
    let mut nums: Vec<i32> = vec![];
    let mut value = 0;

    for (_i, l) in lines.enumerate() {
        let t: Vec<_> = l.chars().filter(|c| c.is_digit(10)).collect();
        // println!("{:?}", t);
        let mut hello = String::new();
        hello.push(*t.first().unwrap());
        hello.push(*t.last().unwrap());
        let my_int = hello.parse::<i32>().unwrap();
        nums.push(my_int);
    }
    
    for n in &nums {
        value = value + n;
    }
    println!("{}",value);
}
