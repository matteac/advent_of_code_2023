pub fn solve() {
    let input = std::fs::read_to_string("input/day_02.txt").unwrap();
    let _example: String = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        .into();
    println!("Day 2, part 1: {}", part1(input.clone()));
    println!("Day 2, part 1: {}", part2(input));
}

fn part1(input: String) -> u32 {
    let games = input.lines();
    let (r, g, b) = (12, 13, 14);
    let mut possible_games = vec![];
    for game in games {
        let mut possible = true;
        let (game_id, plays) = game.split_once(':').unwrap();
        let plays = plays.split(';').collect::<Vec<&str>>();
        for play in plays {
            let blocks = play.split(',').collect::<Vec<&str>>();
            for block in blocks {
                let (mut rs, mut gs, mut bs) = (0, 0, 0);
                let (number, color) = block.trim().split_once(' ').unwrap();
                match color.to_lowercase().as_str() {
                    "red" => rs += number.parse::<u32>().unwrap(),
                    "green" => gs += number.parse::<u32>().unwrap(),
                    "blue" => bs += number.parse::<u32>().unwrap(),
                    _ => {
                        unreachable!()
                    }
                }
                if rs > r || gs > g || bs > b {
                    possible = false;
                }
            }
        }
        let game_id = game_id.replace("Game ", "").parse::<u32>().unwrap();
        if possible {
            possible_games.push(game_id);
        }
    }
    let mut sum = 0;
    for game in possible_games {
        sum += game
    }
    sum
}

fn part2(input: String) -> u32 {
    let games = input.lines();
    let mut sum = 0;
    for game in games {
        let (mut mr, mut mg, mut mb) = (0, 0, 0);
        let (_, plays) = game.split_once(':').unwrap();
        let plays = plays.split(';').collect::<Vec<&str>>();
        for play in plays {
            let blocks = play.split(',').collect::<Vec<&str>>();
            for block in blocks {
                let (number, color) = block.trim().split_once(' ').unwrap();
                let number = number.parse::<u32>().unwrap();
                match color.to_lowercase().as_str() {
                    "red" => {
                        if number > mr {
                            mr = number;
                        }
                    }
                    "green" => {
                        if number > mg {
                            mg = number;
                        }
                    }
                    "blue" => {
                        if number > mb {
                            mb = number;
                        }
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
        }
        sum += mr * mg * mb;
    }
    sum
}
