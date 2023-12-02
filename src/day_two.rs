use crate::inputs;
use std::str::FromStr;

#[derive(Debug)]
struct CubeSet {
    red: u32,
    blue: u32,
    green: u32,
}

impl CubeSet {
    fn new(red: u32, green: u32, blue: u32) -> CubeSet {
        CubeSet { red, green, blue }
    }

    fn get_power(self: &Self) -> u32 {
        return self.red * self.blue * self.green;
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn new(id: u32, rounds: Vec<Round>) -> Game {
        Game { id, rounds }
    }

    fn game_possible(self: &Self, limit_cube_set: &CubeSet) -> bool {
        self.rounds
            .iter()
            .all(|round| round.round_possible(limit_cube_set))
    }

    fn get_minimum_cube_set(self: &Self) -> CubeSet {
        CubeSet::new(
            self.rounds
                .iter()
                .map(|round| round.result.red)
                .max()
                .unwrap(),
            self.rounds
                .iter()
                .map(|round| round.result.green)
                .max()
                .unwrap(),
            self.rounds
                .iter()
                .map(|round| round.result.blue)
                .max()
                .unwrap(),
        )
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(game_str: &str) -> Result<Game, String> {
        // Reference string
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let parts: Vec<&str> = game_str.split(":").map(|str| str.trim()).collect();

        if parts.len() != 2 {
            return Err(String::from("Invalid string for game"));
        }

        let game_id_str = parts.first().unwrap();
        let rounds_str = parts.last().unwrap();

        let game_id_chunks: Vec<&str> = game_id_str.split(" ").collect();
        let last_chunk = match game_id_chunks.last() {
            Some(chunk) => chunk,
            None => return Err(String::from("Invalid string for game")),
        };

        let game_id = match last_chunk.parse::<u32>() {
            Ok(val) => val,
            Err(_e) => return Err(String::from("Invalid string for game")),
        };

        let rounds: Vec<Round> = rounds_str
            .split(";")
            .map(|str| str.trim())
            .map(|str| Round::from_str(str).unwrap()) // TODO - figure out how to get rid of this nicely
            .collect();

        Ok(Game::new(game_id, rounds))
    }
}

#[derive(Debug)]
struct Round {
    result: CubeSet,
}

impl Round {
    fn new(red: u32, green: u32, blue: u32) -> Round {
        Round {
            result: CubeSet::new(red, green, blue),
        }
    }

    fn round_possible(self: &Self, limit_cube_set: &CubeSet) -> bool {
        self.result.red <= limit_cube_set.red
            && self.result.green <= limit_cube_set.green
            && self.result.blue <= limit_cube_set.blue
    }
}

impl FromStr for Round {
    type Err = String;

    fn from_str(round_str: &str) -> Result<Round, String> {
        let cube_strings: Vec<&str> = round_str.split(",").map(|str| str.trim()).collect();

        if cube_strings.len() > 3 {
            return Err(String::from("Invalid string for round"));
        }

        let mut round: Round = Round::new(0, 0, 0);

        let result = cube_strings.iter().try_for_each(|cube_result_str| {
            // Split parts of the single result with ' '
            let parts: Vec<&str> = cube_result_str.split(" ").map(|str| str.trim()).collect();
            if parts.len() != 2 {
                return Err(String::from("Invalid string for round"));
            }

            // Parse colour count
            let count = match parts.first().unwrap().parse::<u32>() {
                Ok(count) => count,
                Err(_e) => return Err(String::from("Invalid string for round")),
            };

            let color = parts.last().unwrap().to_owned();

            // Add colour to existing round data
            match color {
                "red" => {
                    round = Round::new(
                        round.result.red + count,
                        round.result.green,
                        round.result.blue,
                    )
                }
                "green" => {
                    round = Round::new(
                        round.result.red,
                        round.result.green + count,
                        round.result.blue,
                    )
                }
                "blue" => {
                    round = Round::new(
                        round.result.red,
                        round.result.green,
                        round.result.blue + count,
                    )
                }
                _ => return Err(String::from("Invalid string for round, colour not found")),
            };

            Ok(())
        });

        match result {
            Ok(()) => Ok(round),
            Err(e) => Err(e),
        }
    }
}

pub(crate) fn part_one() {
    println!("Day Two, Part One");
    let lines = inputs::read_inputs_from_file("./inputs/day_two.txt").unwrap();

    let games: Vec<Game> = lines
        .iter()
        .map(|line| Game::from_str(line).unwrap())
        .collect();

    let valid_games: Vec<&Game> = games
        .iter()
        .filter(|game| game.game_possible(&CubeSet::new(12, 13, 14)))
        .collect();

    let valid_game_id_sum = valid_games
        .iter()
        .map(|game| game.id)
        .reduce(|acc, game| acc + game)
        .unwrap();

    println!("Sum of valid games: {}", valid_game_id_sum);
}

pub(crate) fn part_two() {
    println!("Day Two, Part Two");
    let lines = inputs::read_inputs_from_file("./inputs/day_two.txt").unwrap();

    let games: Vec<Game> = lines
        .iter()
        .map(|line| Game::from_str(line).unwrap())
        .collect();

    let minimum_cube_sets: Vec<CubeSet> = games
        .iter()
        .map(|game| game.get_minimum_cube_set())
        .collect();

    let power_sum: u32 = minimum_cube_sets
        .iter()
        .map(|cube_set| cube_set.get_power())
        .sum();

    println!("Sum of game minimum possible cube powers: {}", power_sum);
}
