use std::u128::MAX;

struct Game {
    id: usize,
    max_red: usize,
    max_green: usize,
    max_blue: usize,
}

// The possible numbers of cubes are:
// 12 red cubes, 13 green cubes, and 14 blue cubes

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    let lines = input.lines();
    let mut gamelist: Vec<usize> = vec![];
    const MAX_GAME: Game = Game {
        id: 0,
        max_red: 12,
        max_green: 13,
        max_blue: 14,
    };

    for (_i, line) in lines.enumerate() {
        // Remote Whitespace
        let data: String = line.chars().filter(|c| !c.is_whitespace()).collect();
        // let data: String = line.to_string();
        let gamesep = data.find(':').unwrap();

        // Cut off "Game"
        let gameid = &data["Game".len()..gamesep];
        let data = &data[gamesep + 1..data.len()];
        // println!("gameid: {:?}", gameid);

        let mut game = Game {
            id: gameid.parse().unwrap(),
            max_red: 0,
            max_green: 0,
            max_blue: 0,
        };

        let rounds = data.split(';');
        for (_i, round) in rounds.enumerate() {
            for (_i, draw) in round.split(',').enumerate() {
                let split = draw.find(|c: char| !c.is_ascii_digit()).unwrap();
                let num: usize = draw[0..split].parse::<usize>().expect("welp this failed");
                let color = &draw[split..draw.len()];
                // println!("color: {:?}", color);
                // println!("num: {:?}", num);
                //write_to_game(num, color, game);
                match color {
                    "red" => {
                        if game.max_red < num {
                            game.max_red = num;
                        }
                    }
                    "green" => {
                        if game.max_green < num {
                            game.max_green = num;
                        }
                    }
                    "blue" => {
                        if game.max_blue < num {
                            game.max_blue = num
                        }
                    }
                    &_ => todo!(),
                }
            }
        }

        // println!(
        //     "Final of Game:\n id: {}\nMaxRed: {}\nMaxGreen: {}\nMaxBlue: {}\n",
        //     game.id, game.max_red, game.max_green, game.max_blue
        // );
        if game.max_red <= MAX_GAME.max_red
            && game.max_green <= MAX_GAME.max_green
            && game.max_blue <= MAX_GAME.max_blue
        {
            gamelist.push(game.id);
            // println!("Gamelist: {:?}", gamelist);
        }
    }
    println!("Final Sum: {}", gamelist.iter().sum::<usize>())
}
