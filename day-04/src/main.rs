#![allow(unused)]
#![allow(unused_variables)]

#[derive(Clone)]
struct Card {
    id: usize,
    win_nums: Vec<usize>,
    nums: Vec<usize>,
    copies: usize,
}

fn main() {
    // let input = include_str!("../input");
    let input = include_str!("../example");
    let input = input.lines();
    let mut cards: Vec<Card> = vec![];

    for line in input {
        // println!("{line}");
        let line = &line[line.find(|n: char| n.is_ascii_digit()).unwrap()..];
        let id = &line[..line.find(':').unwrap()].parse().unwrap();
        let win_nums: Vec<usize> = line[line.find(' ').unwrap()..line.find(" |").unwrap()]
            .split(' ')
            .flat_map(|s| s.parse::<usize>())
            .collect();
        let nums: Vec<usize> = line[line.find("| ").unwrap()..]
            .split(' ')
            .flat_map(|s| s.parse::<usize>())
            .collect();
        let card = Card {
            id: *id,
            win_nums,
            nums,
            copies: 1,
        };
        // println!("Card {}: {:?} | {:?}", card.id, card.win_nums, card.nums);
        cards.push(card);
    }

    let mut total_value = 0;
    for (_i, card) in cards.iter().enumerate() {
        if card.win_nums.iter().any(|n| card.nums.contains(n)) {
            let mut win_count = 0;
            for i in &card.nums {
                if card.win_nums.contains(i) {
                    win_count += 1;
                }
            }
            total_value += 2_i32.pow(win_count - 1);
            println!(
                "Card {} Value: {}, Current Total: {}",
                card.id,
                2_i32.pow(win_count - 1),
                total_value
            );
        }
    }
    println!("P1 Final Total: {total_value}");
}
