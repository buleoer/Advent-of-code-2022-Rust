
use std::fmt::Debug;
use std::path::Path;
use std::fs;
use std::cmp::{min, max};
use itertools::Itertools;
use std::ops::{Index, IndexMut};
use std::fmt;


#[derive(Debug, PartialEq)]
pub struct Position{ x: usize, y: usize}

#[derive(Debug)]
pub struct InputData (
    Vec<Vec<Position>>,
);

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellContent {
    Air,
    Wall,
    Sand,
}


impl CellContent {
    pub fn to_char(&self) -> char {
        match self {
            Self::Air => ' ',
            Self::Wall => '#',
            Self::Sand => 'o',
        }
    }
}

impl fmt::Display for CellContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}


#[derive(Default)]
struct World {
    xmin: usize,
    xmax: usize,
    row_size: usize,
    ymax: usize,
    cells: Box<[CellContent]>,
}


impl Index<(usize, usize)> for World {
    type Output = CellContent;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.cells[self.from_coords_to_indice(x, y)]
    }
}

impl IndexMut<(usize, usize)> for World {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.cells[self.from_coords_to_indice(x, y)]
    }
}


impl World {

    #[inline]
    fn from_coords_to_indice(&self, x: usize, y: usize) -> usize {
        y * self.row_size + (x - self.xmin)
    }

    // Create an empty world (filled of CellContent::Air) of a given size
    fn create_empty_world(xmin: usize, xmax: usize, ymax: usize) -> Self {
        let cells_size: usize = (xmax - xmin + 1) * (ymax + 1 + 2);
        let mut w = World {
            xmin,
            xmax,
            row_size: xmax - xmin + 1,
            ymax,
            cells: vec!(CellContent::Air; cells_size).into_boxed_slice(),
        };

        // Bottom wall
        for x in xmin..=xmax {
            w[(x, ymax)] = CellContent::Wall;
        };

        w
    }

    fn set_walls(&mut self, walls: &Vec<Vec<Position>>) {
        for wall in walls {
            for (p1, p2) in wall.iter().tuple_windows::<(_, _)>() {
                if p1.x == p2.x {
                    for y in min(p1.y, p2.y)..=max(p1.y, p2.y) {
                        self[(p1.x, y)] = CellContent::Wall;
                    }
                }
                else if p1.y == p2.y {
                    for x in min(p1.x, p2.x)..=max(p1.x, p2.x) {
                        self[(x, p1.y)] = CellContent::Wall;
                    }
                }
                else {
                    panic!("Invalid segment");
                }
                //print!("{}", self);
            }
        }
    }

    pub fn from_file<P: AsRef<Path>>(filename: P) -> Self {
        let input = fs::read_to_string(filename).unwrap();
        let (_, walls) = parser::parse_world(&input).unwrap();

        // Get the height of the world and use it to calculate the boundaries
        let mut ymax = usize::MIN;
        walls.iter().flatten().for_each(|Position{ x:_, y }| {
            ymax = max(ymax, *y);
        });
        ymax = ymax + 2;
        let xmin: usize = 500 - (ymax - 1);
        let xmax: usize = 500 + (ymax + 1);

        let mut w = Self::create_empty_world(xmin, xmax, ymax);

        w.set_walls(&walls);

        w
    }


    fn drop_sand(&mut self, pos: Position) -> bool {

        let mut y = pos.y;
        while (y < self.ymax) && (self[(pos.x, y + 1)] == CellContent::Air) {
            y += 1;
        }
        if y == self.ymax - 1 {
            self[(pos.x, y)] = CellContent::Sand;
            return true;
        }

        // to the left
        if pos.x == self.xmin {
            return true;
        }
        if self[(pos.x - 1, y + 1)] == CellContent::Air {
            return self.drop_sand(Position{ x: pos.x-1, y: y+1 });
        }
        
        // to the right
        if pos.x == self.xmax {
            return true;
        }
        if self[(pos.x + 1, y + 1)] == CellContent::Air {
            return self.drop_sand(Position{ x: pos.x+1, y: y+1 });
        }

        self[(pos.x, y)] = CellContent::Sand;
        false
    }

    pub fn run_part_1(&mut self) -> u32 {
        let mut count = 0u32;
        while !self.drop_sand(Position{ x: 500, y: 0 }) {
            count += 1;
        }
        count
    }

    pub fn run_part_2(&mut self) -> u32 {
        let mut count = 0u32;
        while self[(500, 0)] == CellContent::Air {
            self.drop_sand(Position{ x: 500, y: 0 });
            count += 1;
        }
        count
    }

}


impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::with_capacity((self.row_size + 1) * (self.ymax + 1));
        for y in 0..=self.ymax {
            for x in self.xmin..=self.xmax {
                output.push(self[(x, y)].to_char());
            }
            output.push('\n')
        }
        writeln!(f, "{}", output)
    }
}




mod parser {

    use super::Position;

    use nom::IResult;
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, u32, newline};
    use nom::sequence::separated_pair;
    use nom::multi::separated_list1;
    use nom::combinator::{map, complete};

    fn parse_position(input: &str) -> IResult<&str, Position> {
        map(
            separated_pair(
                u32,
                char(','),
                u32
            ),
            |(x, y)| Position{ x: x as usize, y: y as usize }
        )(input)
    }

    fn parse_wall(input: &str) -> IResult<&str, Vec<Position>> {
        separated_list1(
            tag(" -> "),
            parse_position
        )(input)
    }

    pub fn parse_world(input: &str) -> IResult<&str, Vec<Vec<Position>>> {
        complete(
            separated_list1(
                newline,
                parse_wall
            )
        )(input)
    }

    #[cfg(test)]
    mod test {
        use super::Position;

        #[test]
        fn test_parse_world(){
            assert_eq!(
                super::parse_world("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9\n"),
                Ok(("\n",
                    vec![
                        vec![Position{x:498, y:4}, Position{x:498, y:6}, Position{x:496, y:6}],
                        vec![Position{x:503, y:4}, Position{x:502, y:4}, Position{x:502, y:9}, Position{x:494, y:9}]
                    ]
                ))
            );
        }
    }
}


pub fn run_part_1<P: AsRef<Path>>(filename: P) -> u32 {
    let mut w = World::from_file(filename);
    w.run_part_1()
}

pub fn run_part_2<P: AsRef<Path>>(filename: P) -> u32 {
    let mut w = World::from_file(filename);
    w.run_part_2()
}





#[cfg(test)]
mod test {

    #[test]
    fn test_part_1() {
        let mut w = super::World::from_file("test_input/day14.txt");
        assert_eq!(w.run_part_1(), 24);
    }

    #[test]
    fn test_part_2() {
        let mut w = super::World::from_file("test_input/day14.txt");
        assert_eq!(w.run_part_2(), 93);
    }

}

