use std::{fs::File, path::Path};
use std::io::{self, BufRead};

use std::ops::RangeInclusive;

use nom::IResult;
use nom::character::complete::{char, u32};
use nom::sequence::separated_pair;
use nom::combinator::map;


pub fn run_part_1<P: AsRef<Path>>(filename: P) -> u32 {
    let file = File::open(filename).unwrap();
    let mut count_contained_ranges = 0u32;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let (_, (r1, r2)) = parse_line(&line).unwrap();
        if contains(&r1, &r2) || contains(&r2, &r1) {
            count_contained_ranges += 1;
        }
    }
    count_contained_ranges
}


pub fn run_part_2<P: AsRef<Path>>(filename: P) -> u32 {
    let file = File::open(filename).unwrap();
    let mut count_overlapping_ranges = 0u32;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let (_, (r1, r2)) = parse_line(&line).unwrap();
        if overlap(&r1, &r2) {
            count_overlapping_ranges += 1;
        }
    }
    count_overlapping_ranges
}



fn parse_line(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    separated_pair(
        parse_range,
        char(','),
        parse_range
    )(input)
}


fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    map(
        separated_pair(
            u32,
            char('-'),
            u32
        ),
        |(start, end)| RangeInclusive::new(start, end)
    )(input)
}


fn contains(r1: &RangeInclusive<u32>, r2: &RangeInclusive<u32>) -> bool {
    r1.start() <= r2.start() && r1.end() >= r2.end()
}

fn overlap(r1: &RangeInclusive<u32>, r2: &RangeInclusive<u32>) -> bool {
    contains(r1, r2)
    || contains(r2, r1)
    || (r1.start() <= r2.start() && r1.end() >= r2.start())
    || (r1.start() <= r2.end() && r1.end() >= r2.end())
}


#[cfg(test)]
mod test {

    #[test]
    fn test_part1() {
        assert_eq!(super::run_part_1("test_input/day04.txt"), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::run_part_2("test_input/day04.txt"), 4);
    }

}

