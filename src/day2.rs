use std::{io::{Read, BufRead, BufReader}, fmt::Debug};

use crate::util;


#[derive(Debug)]
struct Game {
    pub game_num: u32,
    pub hands: Vec<Hand>,
}

struct Hand {
    pub red_count: u16,
    pub green_count: u16,
    pub blue_count: u16,
}

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hand").field("r", &self.red_count).field("g", &self.green_count).field("b", &self.blue_count).finish()
    }
}

fn str2counts(hand_str: &str) -> Hand {
    let raw_hand: Vec<(String, u16)> = hand_str.split(',')
            .map(|s| {
                let num = s.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<u16>().unwrap();
                let color = s.chars().filter(|c| c.is_alphabetic()).collect::<String>();

                (color, num)
            }).collect();

    let &(_,red_count) = raw_hand.iter().find(|&h| h.0.eq_ignore_ascii_case("red")).unwrap_or(&("red".into(), 0));
    let &(_,green_count) = raw_hand.iter().find(|&h| h.0.eq_ignore_ascii_case("green")).unwrap_or(&("green".into(), 0));
    let &(_,blue_count) = raw_hand.iter().find(|&h| h.0.eq_ignore_ascii_case("blue")).unwrap_or(&("blue".into(), 0));

    Hand { red_count, green_count, blue_count }
}

pub fn day2(file: impl Read, (red_limit, green_limit, blue_limit): (u16, u16, u16)) -> u32 {
    let mut lines = util::read_lines(file);
    let mut accepted_games = Vec::<Game>::new();

    while let Some(Ok(line)) = lines.next() {
        let game = parse_game(&line);
        if game.hands.iter().all(|h| h.blue_count <= blue_limit && h.green_count <= green_limit && h.red_count <= red_limit) {
            accepted_games.push(game);
        }
    }

    accepted_games.iter().map(|g| g.game_num).sum()
}

pub fn day2_pt2(file: impl Read) -> u64 {
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut powers = Vec::<u64>::new();

    while let Some(Ok(line)) = lines.next() {
        let game = parse_game(&line);
        let power = get_game_power(&game);

        powers.push(power);
    }

    powers.iter().sum()
}

fn get_game_power(game: &Game) -> u64 {
    let hands = &game.hands;

    let (highest_red, highest_green, highest_blue) = hands.iter().fold((0,0,0), |mut s, h| {
        if h.red_count as u64 > s.0 {
            s.0 = h.red_count as u64;
        }

        if h.green_count as u64 > s.1 {
            s.1 = h.green_count as u64;
        }

        if h.blue_count as u64 > s.2 {
            s.2 = h.blue_count as u64;
        }

        s
    });

    highest_red * highest_green * highest_blue
}

fn parse_game(line: &str) -> Game {
    let split = line.split(':').collect::<Vec<&str>>();
    let num_string = split.first().unwrap().chars().filter(|c| c.is_ascii_digit()).collect::<String>();
    let game_num = num_string.parse::<u32>().unwrap();

    let hands_split = split.last().unwrap().split(';').map(str2counts).collect();

    Game { game_num, hands: hands_split }
}

#[cfg(test)]
mod tests {
    use crate::util;

    const DAY2_INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    const TEST_INPUT: &str = "Game 1: 1 green, 2 blue; 15 blue, 12 red, 2 green; 4 red, 6 blue; 10 blue, 8 red; 3 red, 12 blue; 1 green, 12 red, 8 blue";

    #[test]
    fn day2() {
        let input = util::str2reader(DAY2_INPUT);

        let res = super::day2(input, (12,13,14));

        assert_eq!(8, res);
    }

    #[test]
    fn day2_pt2() {
        let input = util::str2reader(DAY2_INPUT);

        let power_sum = super::day2_pt2(input);

        assert_eq!(2286, power_sum);
    }

    #[test]
    fn input_test() {
        let game = super::parse_game(TEST_INPUT);

        assert_eq!(6, game.hands.len());
        assert_eq!(1, game.game_num);
    }
}