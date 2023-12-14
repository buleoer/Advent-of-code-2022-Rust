
use std::{path::Path, fs::File};
use std::io::{self, BufRead};

use nom::IResult;
use nom::character::complete::{char, one_of};
use nom::sequence::separated_pair;
use nom::combinator::map;


#[derive(Debug)]
enum Play {
    Rock, Paper, Scissor
}


impl Play {
    
    pub fn from_opponent(code: char) -> Self {
        match code {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissor,
            _ => panic!("Invalid value!"),
        }
    }

    pub fn from_me(code: char) -> Self {
        match code {
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissor,
            _ => panic!("Invalid value!"),
        }
    }

    pub fn play_part_1(my_play: &Play, opponent_play: &Play) -> (PlayResult, u32) {
        match (my_play, opponent_play) {
            (Self::Rock, Self::Rock) => (PlayResult::Draw, 1 + 3),
            (Self::Rock, Self::Paper) => (PlayResult::Lost, 1 + 0),
            (Self::Rock, Self::Scissor) => (PlayResult::Won, 1 + 6),
            (Self::Paper, Self::Rock) => (PlayResult::Won, 2 + 6),
            (Self::Paper, Self::Paper) => (PlayResult::Draw, 2 + 3),
            (Self::Paper, Self::Scissor) => (PlayResult::Lost, 2 + 0),
            (Self::Scissor, Self::Rock) => (PlayResult::Lost, 3 + 0),
            (Self::Scissor, Self::Paper) => (PlayResult::Won, 3 + 6),
            (Self::Scissor, Self::Scissor) => (PlayResult::Draw, 3 + 3),
        }
    }

    pub fn play_part_2(opponent_play: &Play, expected_result: &PlayResult) -> (Play, u32) {
        match (opponent_play, expected_result) {
            (Self::Rock, PlayResult::Won) => (Self::Paper, 2 + 6),
            (Self::Rock, PlayResult::Draw) => (Self::Rock, 1 + 3),
            (Self::Rock, PlayResult::Lost) => (Self::Scissor, 3 + 0),
            (Self::Paper, PlayResult::Won) => (Self::Scissor, 3 + 6),
            (Self::Paper, PlayResult::Draw) => (Self::Paper, 2 + 3),
            (Self::Paper, PlayResult::Lost) => (Self::Rock, 1 + 0),
            (Self::Scissor, PlayResult::Won) => (Self::Rock, 1 + 6),
            (Self::Scissor, PlayResult::Draw) => (Self::Scissor, 3 + 3),
            (Self::Scissor, PlayResult::Lost) => (Self::Paper, 2 + 0),
        }
    }

}

#[derive(Debug)]
enum PlayResult {
    Won, Draw, Lost
}

impl PlayResult {
    pub fn from_code_for_part_2(code: char) -> Self {
        match code {
            'X' => Self::Lost,
            'Y' => Self::Draw,
            'Z' => Self::Won,
            _ => panic!("Invalid value!"),
        }
    }
}


pub fn run_part_1<P: AsRef<Path>>(filename: P) -> u32 {
    let file = File::open(filename).unwrap();
    let mut total_score = 0u32;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let (_, (opponent_play, my_play)) = parse_line_part_1(&line).unwrap();
        let (_play_result, score) = Play::play_part_1(&my_play, &opponent_play);
        total_score += score;
    }
    total_score
}


pub fn run_part_2<P: AsRef<Path>>(filename: P) -> u32 {
    let file = File::open(filename).unwrap();
    let mut total_score = 0u32;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let (_, (opponent_play, expected_outcome)) = parse_line_part_2(&line).unwrap();
        let (_my_play, score) = Play::play_part_2(&opponent_play, &expected_outcome);
        total_score += score;
    }
    total_score
}


fn parse_line_part_1(input: &str) -> IResult<&str, (Play, Play)> {
    separated_pair(
        map(one_of("ABC"), |c| Play::from_opponent(c)),
        char(' '),
        map(one_of("XYZ"), |c| Play::from_me(c))
    )(input)
}

fn parse_line_part_2(input: &str) -> IResult<&str, (Play, PlayResult)> {
    separated_pair(
        map(one_of("ABC"), |c| Play::from_opponent(c)),
        char(' '),
        map(one_of("XYZ"), |c| PlayResult::from_code_for_part_2(c))
    )(input)
}


#[cfg(test)]
mod test {

    #[test]
    fn test_part1() {
        assert_eq!(super::run_part_1("test_input/day02.txt"), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::run_part_2("test_input/day02.txt"), 12);
    }

}

