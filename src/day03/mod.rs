
use std::{path::Path, fs::File};
use std::io::{self, BufRead, BufReader};

use itertools::Itertools;


pub fn run_part_1<P: AsRef<Path>>(filename: P) -> u32 {
    let file = File::open(filename).unwrap();
    let mut total_priority = 0u32;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let length = line.len();
        let (part1, part2) = (&line[0..length/2], &line[length/2..length]);
        let common_item = find_common_item(part1, part2).unwrap();
        let priority = get_item_priority(&common_item).unwrap();
        // println!("{:?} {:?}", common_item, priority);
        total_priority += priority;
    }
    total_priority
}


pub fn run_part_2<P: AsRef<Path>>(filename: P) -> u32 {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
        .lines()
        .map(|s| s.unwrap())
        .tuples::<(_, _, _)>()
        .map(|(s1, s2, s3)| find_common_item_part_2(s1, s2, s3).unwrap())
        .map(|c| get_item_priority(&c).unwrap())
        .sum()
}


fn find_common_item(s1: &str, s2: &str) -> Option<char> {
    for c in s1.chars() {
        if s2.contains(c) {
            return Some(c);
        }
    }
    None
}

pub fn find_common_item_part_2(s1: String, s2: String, s3: String) -> Option<char> {
    for c in s1.chars() {
        if s2.contains(c) && s3.contains(c) {
            return Some(c);
        }
    }
    None
}

fn get_item_priority(item: &char) -> Result<u32, &str> {
    if item.is_ascii_lowercase() {
        return Ok(*item as u32 - 'a' as u32 + 1);
    }
    else if item.is_ascii_uppercase() {
        return Ok(*item as u32 - 'A' as u32 + 27)
    }
    Err("Invalid character!")
}


#[cfg(test)]
mod test {

    #[test]
    fn test_part1() {
        assert_eq!(super::run_part_1("test_input/day03.txt"), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::run_part_2("test_input/day03.txt"), 70);
    }

}

