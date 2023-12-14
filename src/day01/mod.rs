use std::{fs::File, path::Path};
use std::io::{self, BufRead};


pub fn run_part_1<P: AsRef<Path>>(filename: P) -> u32 {
    let file = File::open(filename).unwrap();
    let mut calories_max = 0u32;
    let mut calories = 0u32;
    for line in io::BufReader::new(file).lines() {
        match line.unwrap().as_str() {
            "" =>  {
                calories_max = calories_max.max(calories);
                calories = 0;
            },
            svalue => {
                let v = svalue.parse::<u32>().unwrap();
                calories += v;
            },
        }
    }
    calories_max = calories_max.max(calories);
    calories_max
}


pub fn run_part_2<P: AsRef<Path>>(filename: P) -> u32 {
    let file = File::open(filename).unwrap();
    let mut elves_calories: Vec<u32> = Vec::new();
    let mut calories = 0u32;
    for line in io::BufReader::new(file).lines() {
        match line.unwrap().as_str() {
            "" =>  {
                elves_calories.push(calories);
                calories = 0;
            },
            svalue => {
                let v = svalue.parse::<u32>().unwrap();
                calories += v;
            },
        }
    }
    elves_calories.push(calories);
    elves_calories.sort_unstable_by(|a, b| {
        b.partial_cmp(a).unwrap()
    });
    elves_calories.get(0).unwrap()
        + elves_calories.get(1).unwrap()
        + elves_calories.get(2).unwrap()
}




#[cfg(test)]
mod test {

    #[test]
    fn test_part1() {
        assert_eq!(super::run_part_1("test_input/day01.txt"), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::run_part_2("test_input/day01.txt"), 45000);
    }
}

