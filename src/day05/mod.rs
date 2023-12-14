use std::{fs, path::Path};

use nom::IResult;

use nom::character::complete::{char, u16, satisfy, line_ending};
use nom::bytes::complete::{tag, is_not};
use nom::sequence::{tuple, delimited, terminated};
use nom::branch::alt;
use nom::combinator::{map, verify};
use nom::multi::{many1, separated_list1};


struct Move {
    count: usize,
    from_stack: usize,      // between 1 and N
    to_stack: usize,        // between 1 and N
}

struct SupplyStacks <const N: usize> {
    stacks: [Vec<char>; N],
    moves: Vec<Move>,
}

impl <const N: usize> SupplyStacks<N> {

    pub fn do_moves(stacks: &mut [Vec<char>; N], moves: &Vec<Move>) {
        for m in moves {
            for _ in 0..m.count {
                let c = stacks[m.from_stack - 1].pop().unwrap();
                stacks[m.to_stack - 1].push(c);
            }
        }
    }

    pub fn do_moves_part_2(stacks: &mut [Vec<char>; N], moves: &Vec<Move>) {
        for m in moves {
            assert!(m.from_stack > 0 && m.from_stack <= N);
            assert!(m.to_stack > 0 && m.to_stack <= N);
            assert!(m.from_stack != m.to_stack);
            
            let truncate_start_position = stacks[m.from_stack - 1].len() - m.count;
            let mut crates_to_move: Vec<_> = stacks[m.from_stack - 1].drain(truncate_start_position..).collect();
            // Note: Rust does not allow mutable borrowing of 2 elements of an array without unsafe
            // so we cannot drain directly from a stack to another as they are in the same array "stacks".

            stacks[m.to_stack - 1].append(&mut crates_to_move);
        }
    }


    pub fn get_result(&self) -> String {
        let mut result = String::with_capacity(N);
        for stack in &self.stacks {
            result.push(*stack.last().unwrap());
        }
        result
    }
}



pub fn run_part_1<P: AsRef<Path>>(filename: P) -> String {
    let input = fs::read_to_string(filename).unwrap();
    let (_, mut game) = parse_all::<9>(&input).unwrap();
    SupplyStacks::do_moves(&mut game.stacks, &game.moves);
    game.get_result()
}


pub fn run_part_2<P: AsRef<Path>>(filename: P) -> String {
    let input = fs::read_to_string(filename).unwrap();
    let (_, mut game) = parse_all::<9>(&input).unwrap();
    SupplyStacks::do_moves_part_2(&mut game.stacks, &game.moves);
    game.get_result()
}


fn parse_crate(input: &str) -> IResult<&str, Option<char>> {
    alt((
        delimited(
            char('['),
            map(
                satisfy(|c| c.is_ascii_uppercase()),
                |c| Some(c)
            ),
            char(']')
        ),
        map(
            tag("   "),
            |_| None
        )
    ))(input)
}

fn parse_crate_layer<const N: usize>(input: &str) -> IResult<&str, Vec<Option<char>>> {
    verify(
        separated_list1(char(' '), parse_crate),
        |v: &Vec<_>| v.len() == N
    )(input)
}

fn parse_crates<const N: usize>(input: &str) -> IResult<&str, [Vec<char>; N]> {
    map(
        many1(
            terminated(parse_crate_layer::<N>, line_ending)
        ),
        |v| {
            //println!(" coucou: {:?}", v);
            let mut result: [Vec<char>; N] = std::array::from_fn(|_| Vec::new());
            for stack in v.iter().rev() {
                for (i, crat) in stack.iter().enumerate() {
                    match crat {
                        Some(c) => result[i].push(*c),
                        None => (),
                    }
                }
            }
            result
        }
    )(input)
}


fn parse_move(input: &str) -> IResult<&str, Move> {
    map(
        tuple((
            tag("move "),
            u16,
            tag(" from "),
            u16,
            tag (" to "),
            u16
        )),
        |(_, count, _, from_stack, _, to_stack)|
            Move {
                count: count as usize,
                from_stack: from_stack as usize,
                to_stack: to_stack as usize
            }
    )(input)
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    many1(
        terminated(parse_move, line_ending)
    )(input)
}

fn parse_all<const N: usize>(input: &str) -> IResult<&str, SupplyStacks<N>> {
    map(
        tuple((
            parse_crates::<N>,
            is_not("\n"),
            char('\n'),
            char('\n'),
            parse_moves,
        )),
        |t| SupplyStacks{stacks: t.0, moves: t.4}
    )(input)
}


#[cfg(test)]
mod test {

    use std::fs;

    #[test]
    fn test_parser() {
        //assert_eq!(super::parse_crate("[A]"), Ok("", Option::<char>::Some('A')));
        assert_eq!(super::parse_crate("[A]"), Ok(("", Some('A'))));
        assert_eq!(super::parse_crate("   "), Ok(("", None)));

        assert_eq!(
            super::parse_crate_layer::<3>("    [D]    "),
            Ok(("", vec!(None, Some('D'), None)))
        );
        assert_eq!(
            super::parse_crate_layer::<3>("[N] [C]    "),
            Ok(("", vec!(Some('N'), Some('C'), None)))
        );

        assert_eq!(
            super::parse_crates::<3>("    [D]    \n[N] [C]    \n[Z] [M] [P]\n"),
            Ok(("", [
                vec!('Z', 'N'),
                vec!('M', 'C', 'D'),
                vec!('P')
            ]))
        );
    }


    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input/day05.txt").unwrap();
        let (_, mut game) = super::parse_all::<3>(&input).unwrap();
        super::SupplyStacks::do_moves(&mut game.stacks, &game.moves);
        assert_eq!(game.get_result(), String::from("CMZ"));
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("test_input/day05.txt").unwrap();
        let (_, mut game) = super::parse_all::<3>(&input).unwrap();
        super::SupplyStacks::do_moves_part_2(&mut game.stacks, &game.moves);
        assert_eq!(game.get_result(), String::from("MCD"));
    }

}


