struct Game {
    id: usize,
    max_red: usize,
    max_green: usize,
    max_blue: usize,
}

// The possible numbers of cubes are:
// 12 red cubes, 13 green cubes, and 14 blue cubes

fn main() {
    const MAX_GAME: Game = Game {
        id: 0,
        max_red: 12,
        max_green: 13,
        max_blue: 14,
    };

    //let input = include_str!("../input");
    let input = include_str!("../example");
    let mut gamelist: Vec<usize> = vec![];

    for (_i, line) in input.lines().enumerate() {
        // Remote Whitespace
        let data: String = line.chars().filter(|c| !c.is_whitespace()).collect();
        // let data: String = line.to_string();
        let gamesep = data.find(':').unwrap();
        // Cut off "Game"
        let gameid = &data["Game".len()..gamesep];
        let data = &data[gamesep + 1..data.len()];
        // println!("gameid: {:?}", gameid);

        // Define game
        let mut game = Game {
            id: gameid.parse().unwrap(),
            max_red: 0,
            max_green: 0,
            max_blue: 0,
        };

        // Parse the data
        for (_i, round) in data.split(';').enumerate() {
            for (_i, draw) in round.split(',').enumerate() {
                let div = draw.find(|c: char| !c.is_ascii_digit()).unwrap();
                let num: usize = draw[0..div]
                    .parse::<usize>()
                    .expect("welp this failed *shrug*");
                let color = &draw[div..draw.len()];

                // Checking if we need to update the max values for the game
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
                    // Checkall because rust told me to
                    &_ => todo!(),
                }
            }
        }

        // println!(
        //     "Final of Game:\n id: {}\nMaxRed: {}\nMaxGreen: {}\nMaxBlue: {}\n",
        //     game.id, game.max_red, game.max_green, game.max_blue
        // );

        // Find games that don't exceede the values of MAX_GAME
        if game.max_red <= MAX_GAME.max_red
            && game.max_green <= MAX_GAME.max_green
            && game.max_blue <= MAX_GAME.max_blue
        {
            gamelist.push(game.id);
            // println!("Gamelist: {:?}", gamelist);
        }
    }
    println!("Part One Final Sum: {}", gamelist.iter().sum::<usize>())
}
