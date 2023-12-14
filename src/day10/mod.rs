use std::{fs::File, path::Path};
use std::io::{BufReader, BufRead};
use std::fmt;

use nom::IResult;
use nom::character::complete::i32;
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::sequence::preceded;
use nom::combinator::map;

#[derive(Debug, PartialEq)]
enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Debug)]
struct CRT<const ROWS: usize, const COLUMNS: usize> {
    pixels: [[bool; COLUMNS];ROWS],
    sprite_column: i32,
    current_pixel_column: usize,
    current_pixel_row: usize,
}

impl<const ROWS: usize, const COLUMNS: usize> Default for CRT<ROWS, COLUMNS> {
    fn default() -> Self {
        Self {
            pixels: [[false; COLUMNS]; ROWS],
            sprite_column: 1,
            current_pixel_column: 0,
            current_pixel_row: 0,
        }
    }
}

impl<const ROWS: usize, const COLUMNS: usize> fmt::Display for CRT<ROWS, COLUMNS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::with_capacity(ROWS*(COLUMNS+1));
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                s.push(if self.pixels[r][c] { '#' } else { '.' })
            }
            s.push('\n');
        }
        f.write_str(&s)
    }
}

impl<const ROWS: usize, const COLUMNS: usize> From<&str> for CRT<ROWS, COLUMNS> {
    fn from(s: &str) -> Self {
        let mut row = 0;
        let mut col = 0;
        let mut crt = CRT::default();
        for c in s.chars() {
            crt.pixels[row][col] = c == '#';
            col += 1;
            if col == COLUMNS {
                row += 1;
                col = 0;
            }
            if row == ROWS {
                break;
            }
        };
        crt
    }
}


impl<const ROWS: usize, const COLUMNS: usize> CRT<ROWS, COLUMNS> {

    fn new() -> Self {
        Self::default()
    }

    fn move_sprite(&mut self, val: i32) {
        self.sprite_column += val;
    }

    // pixel_position starts at 1
    fn set_pixel(&mut self) {
        println!("{}", self);

        let current_pixel_row = self.current_pixel_row;
        let current_pixel_column = self.current_pixel_column;
        self.pixels[current_pixel_row][current_pixel_column] =
                self.current_pixel_column.abs_diff(self.sprite_column as usize) <= 1;

        self.advance_pixel();
    }

    fn advance_pixel(&mut self) {
        self.current_pixel_column += 1;
        if self.current_pixel_column == COLUMNS {
            self.current_pixel_row += 1;
            self.current_pixel_column = 0;
        }
    }
}


fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(tag("noop"), |_| Instruction::Noop),
        map(
            preceded(
                tag("addx "),
                i32
            ),
            |v| Instruction::Addx(v)
        )
    ))(input)
}


pub fn run_part_1<P: AsRef<Path>>(filename: P) -> i32 {
    let file = File::open(filename).unwrap();
    //let mut program: Vec<InstructionExecution> = Vec::new();
    let mut current_cycle = 1u32;
    let mut current_register_value = 1i32;
    let mut signal_strength_sum = 0i32;

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let (_, instr) = parse_instruction(&line).unwrap();

        let cycle_after: u32;
        let register_value_after: i32;

        let current_cycle_mod_40 = (current_cycle + 20) % 40;   // Used to determine if this is an "interesting" cycle
        let interesting_cycle: bool;

        match instr {
            Instruction::Noop => {
                cycle_after = current_cycle + 1;
                register_value_after = current_register_value;
                interesting_cycle = current_cycle_mod_40 == 0;
            },
            Instruction::Addx(val) => {
                cycle_after = current_cycle + 2;
                register_value_after = current_register_value + val;
                interesting_cycle = (current_cycle_mod_40 == 39) || (current_cycle_mod_40 == 0);
            },
        }
        
        if interesting_cycle {
            // We are in an "interesting" cycle (20, 60, 100, etc...)
            let interesting_cycle_number = cycle_after - cycle_after % 10;
            let signal_strength: i32 = (interesting_cycle_number as i32) * current_register_value;
            signal_strength_sum += signal_strength;
        }
        current_cycle = cycle_after;
        current_register_value =register_value_after;

    }
    signal_strength_sum
}

pub fn run_part_2<P: AsRef<Path>>(filename: P) -> String {
    let file = File::open(filename).unwrap();
    let mut crt = CRT::<6, 40>::new();
    for line in BufReader::new(file).lines() {        
        let line = line.unwrap();
        let (_, instr) = parse_instruction(&line).unwrap();

        match instr {
            Instruction::Noop => {
                crt.set_pixel();
            },
            Instruction::Addx(val) => {
                crt.set_pixel();
                crt.set_pixel();
                crt.move_sprite(val) ;
            },
        }
    }
    println!("{}", crt);
    crt.to_string()
}




#[cfg(test)]
mod test {

    #[test]
    fn test_part_1() {
        assert_eq!(super::run_part_1("test_input/day10.txt"), 13140);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            super::run_part_2("test_input/day10.txt"),
            concat!(
                "##..##..##..##..##..##..##..##..##..##..\n",
                "###...###...###...###...###...###...###.\n",
                "####....####....####....####....####....\n",
                "#####.....#####.....#####.....#####.....\n",
                "######......######......######......####\n",
                "#######.......#######.......#######.....\n",
            )
        );
    }
}
