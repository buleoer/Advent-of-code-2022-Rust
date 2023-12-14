use std::hash::Hash;
use std::{fs::File, path::Path};
use std::io::{BufReader, BufRead};
use std::collections::HashSet;
use std::cmp::{min, max};

use nom::IResult;
use nom::sequence::separated_pair;
use nom::character::complete::{char, u32, one_of};
use nom::combinator::map;

enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl TryFrom<char> for Direction {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err("Invalid direction value!"),
        }
    }
}


#[derive(PartialEq, Eq, Hash, Default, Copy, Clone)]
struct Position { x: i32, y: i32 }

struct RopeBridge<const NODE_COUNT: usize> {
    // head -> node_position[0]
    // tail -> node_position[NODE_COUNT-1]
    node_position: [Position; NODE_COUNT],
    tail_visited_positions: HashSet<Position>,
}



impl<const NODE_COUNT: usize> RopeBridge<NODE_COUNT> {

    pub fn new() -> Self {
        Self {
            node_position: [Position::default(); NODE_COUNT],
            tail_visited_positions: HashSet::new(),
        }
    }

    // coordinates:
    //
    // y ^
    //   |
    //   |
    //   .------>
    //          x
    //
    pub fn move_to(&mut self, direction: Direction, steps: u32) {
        
        for _ in 0..steps {

            // Move the head
            match direction {
                Direction::Up => { self.node_position[0].y += 1; },
                Direction::Down => { self.node_position[0].y -= 1; },
                Direction::Left => { self.node_position[0].x -= 1; },
                Direction::Right => { self.node_position[0].x += 1; }
            }

            // move other nodes
            for i in 1..NODE_COUNT {
                if Self::get_distance(&self.node_position[i], &self.node_position[i-1]) > 1 {
                    match self.node_position[i-1].x - self.node_position[i].x {
                        1|2 => self.node_position[i].x += 1,      // moved right
                        -1|-2 => self.node_position[i].x -= 1,     // moved left
                        _ => (),
                    }
                    match self.node_position[i-1].y - self.node_position[i].y {
                        1|2 => self.node_position[i].y += 1,      // moved up
                        -1|-2 => self.node_position[i].y -= 1,     // moved down
                        _ => (),
                    }
                }
            }
            
            self.tail_visited_positions.insert(self.node_position[NODE_COUNT-1].clone());
        }
    }


    fn get_distance(p1: &Position, p2: &Position) -> u32 {
        max(p1.x.abs_diff(p2.x), p1.y.abs_diff(p2.y))
    }

    pub fn get_tail_number_of_visited_positions(&self) -> u32 {
        self.tail_visited_positions.len() as u32
    }

    // For debugging
    #[allow(dead_code)]
    fn display_bridge(&self) {

        // find clipping area
        let mut min_x = 0i32;
        let mut min_y = 0i32;
        let mut max_x = 0i32;
        let mut max_y = 0i32;

        self.node_position.into_iter().for_each(|pos| {
            min_x = min(min_x, pos.x);
            min_y = min(min_y, pos.y);
            max_x = max(max_x, pos.x);
            max_y = max(max_y, pos.y);
        });

        for y in (min_y..max_y+1).rev() {
            for x in min_x..max_x + 1 {
                match (0..NODE_COUNT).position( |i| self.node_position[i] == Position { x: x, y: y } ) {
                    None => eprint!("."),
                    Some(i) => {
                        if i == 0 {
                            eprint!("H");
                        } else {
                            eprint!("{}", i);
                        }
                    }
                }
            }
            eprintln!();
        }
        eprintln!();
    }
    
}

pub fn run_part_1<P: AsRef<Path>>(filename: P) -> u32 {
    let file = File::open(filename).unwrap();
    let mut game = RopeBridge::<2>::new();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let (_, (dir, steps)) = parse_line(&line).unwrap();
        game.move_to(dir, steps);
    }
    game.get_tail_number_of_visited_positions()
}


pub fn run_part_2<P: AsRef<Path>>(filename: P) -> u32 {
    let file = File::open(filename).unwrap();
    let mut game = RopeBridge::<10>::new();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let (_, (dir, steps)) = parse_line(&line).unwrap();
        game.move_to(dir, steps);
    }
    game.get_tail_number_of_visited_positions()
}


fn parse_line(input: &str) -> IResult<&str, (Direction, u32)> {
    separated_pair(
        map(one_of("UDLR"), |c| Direction::try_from(c).unwrap()),
        char(' '),
        u32
    )(input)
}


#[cfg(test)]
mod test {

    #[test]
    fn test_part_1() {
        assert_eq!(super::run_part_1("test_input/day09.txt"), 13);
    }

    #[test]
    fn test_part2() {
        //assert_eq!(super::run_part_2("test_input/day09.txt"), 1);
        assert_eq!(super::run_part_2("test_input/day09b.txt"), 36);
    }
}

