use std::path::Path;
use std::fs;
use std::cmp;

use nom::IResult;
use nom::character::complete::{satisfy, line_ending};
use nom::multi::fold_many_m_n;
use nom::sequence::terminated;

use pathfinding::directed::astar::astar;


static NEIGHBORS: &'static [(i32, i32); 4] =
//    &[(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];
    &[(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct HeightMap<const WIDTH: usize, const HEIGHT: usize> {
    map: [[char; WIDTH]; HEIGHT],
    start: Position,
    end: Position,
}

impl <const WIDTH: usize, const HEIGHT: usize> Default for HeightMap<WIDTH, HEIGHT> {
    fn default() -> Self {
        Self {
            map: [['a'; WIDTH]; HEIGHT],
            start: Position{ x: 0, y: 0 },
            end: Position{ x: 0, y: 0 },
        }
    }
}

impl <const WIDTH: usize, const HEIGHT: usize> HeightMap<WIDTH, HEIGHT> {

    pub fn parse_height_map(input: &str) -> IResult<&str, Self> {
        let mut i_row = 0usize;

        fold_many_m_n(
            HEIGHT,
            HEIGHT,
            terminated(Self::parse_row, line_ending),
            HeightMap::<WIDTH, HEIGHT>::default,
            move |mut hm: HeightMap<WIDTH, HEIGHT>, r| {
                let (row, start_position, end_position) = r;
                hm.map[i_row] = row;
                if let Some (x_start) = start_position {
                    hm.start.x = x_start;
                    hm.start.y = i_row;
                };
                if let Some(x_end) = end_position {
                    hm.end.x = x_end;
                    hm.end.y = i_row;
                };
                i_row += 1;
                hm
            }
        )(input)
    }

    fn parse_row(input: &str)
    -> IResult<&str, ([char; WIDTH], Option<usize>, Option<usize>)> {
        let mut i = 0usize;
        let mut start_position: Option<usize> = Option::None;
        let mut end_position: Option<usize> = Option::None;
        let (x, row) = fold_many_m_n(
            WIDTH, WIDTH,
            satisfy(|c| (c >= 'a' && c <= 'z') || c == 'S' || c == 'E'),
            || ['a'; WIDTH],
            |mut a: [char; WIDTH], c| {
                match c {
                    'a'..='z' => a[i] = c,
                    'S' => {
                        a[i] = 'a';
                        start_position = Some(i);
                    }
                    'E' => {
                        a[i] = 'z';
                        end_position = Some(i);
                    }
                    _ => (),
                }
                i += 1;
                a
            }
        )(input)?;
        Ok((x, (row, start_position, end_position)))
    }


    fn get_neighbors(pos: &Position, predicate: impl Fn(&Position) -> bool) -> Vec<(Position, i32)> {
        let mut neighbors: Vec<(Position, i32)> = Vec::with_capacity(4);
        for (dx, dy) in NEIGHBORS {
            let nx = pos.x as i32 + dx;
            let ny = pos.y as i32 + dy;
            if (nx >= 0) && (nx < WIDTH as i32) && (ny >= 0) && (ny < HEIGHT as i32) {
                let pos = Position{ x: nx as usize, y: ny as usize};
                if predicate(&pos) {
                    neighbors.push((pos, 1));
                }
            }
        }
        neighbors
    }


    pub fn find_path2(&self) -> Option<(Vec<Position>, i32)> {
        let result = astar(
            &self.start,
            |pos| {
                Self::get_neighbors(
                    &pos, 
                    |&Position{ x: n_x, y: n_y }| {
                        // A neighbor is valid only if it is at most one level higher
                        self.map[n_y][n_x] as i32 - self.map[pos.y][pos.x] as i32 <= 1
                    },
                )
            },
            |&Position{x, y}| {
                cmp::max(x.abs_diff(self.end.x), y.abs_diff(self.end.y)) as i32
            },
            |&Position{x, y}| (x == self.end.x) && (y == self.end.y),
        )?;
        Some(result)
    }


    pub fn find_shortest_path(&self) -> Option<i32> {
        let mut shortest_path_length: Option<i32> = None;

        for y in 0..HEIGHT {
            for x in 0..WIDTH {

                if self.map[y][x] == 'a' {
                    if let Some(result) = astar(
                        &Position{x, y},
                        |pos| {
                            let nei = Self::get_neighbors(
                                &pos,
                                |&Position{ x: n_x, y: n_y }| {
                                    // if we are on a 'a' we only accept 'b'
                                    if self.map[pos.y][pos.x] == 'a' {
                                        return self.map[n_y][n_x] == 'b'
                                    }
                                    // if we are on a 'b' we don't accept a 'a'
                                    if self.map[pos.y][pos.x] == 'b' && self.map[n_y][n_x] == 'a' {
                                        return false;
                                    }
                                    // for other cases, same rules as in part 1
                                    self.map[n_y][n_x] as i32 - self.map[pos.y][pos.x] as i32 <= 1
                                },
                            );
                            nei
                        },
                        |&Position{x, y}| {
                            cmp::max(x.abs_diff(self.end.x), y.abs_diff(self.end.y)) as i32
                        },
                        |&Position{x, y}| (x == self.end.x) && (y == self.end.y),
                    )
                    {
                        shortest_path_length = Some(
                            match shortest_path_length {
                                None => result.1,
                                Some(v) => cmp::min(v, result.1)
                            }
                        )
                    }
                }

            }
        }
        shortest_path_length
    }

}


pub fn run_part_1<P: AsRef<Path>>(filename: P) -> i32 {
    let input = fs::read_to_string(filename).unwrap();
    let (_, hm) = HeightMap::<61, 41>::parse_height_map(&input).unwrap();
    let (_path, cost) = hm.find_path2().unwrap();
    cost
}


pub fn run_part_2<P: AsRef<Path>>(filename: P) -> i32 {
    let input = fs::read_to_string(filename).unwrap();
    let (_, hm) = HeightMap::<61, 41>::parse_height_map(&input).unwrap();
    let cost = hm.find_shortest_path().unwrap();
    cost
}


#[cfg(test)]
mod test {

    use std::fs;

    #[test]
    fn test_parse_row() {
        assert_eq!(
            super::HeightMap::<8, 5>::parse_row("abcryxxl"),
            Ok(("", (['a', 'b', 'c', 'r', 'y', 'x', 'x', 'l'], None, None)))
        );
        assert_eq!(
            super::HeightMap::<8, 5>::parse_row("aScryxxl"),
            Ok(("", (['a', 'a', 'c', 'r', 'y', 'x', 'x', 'l'], Some(1), None)))
        );

        assert_eq!(
            super::HeightMap::<8, 5>::parse_row("abcryxEl"),
            Ok(("", (['a', 'b', 'c', 'r', 'y', 'x', 'z', 'l'], None, Some(6))))
        );
    }

    #[test]
    fn test_parse_height_map() {
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("test_input/day12.txt").unwrap();
        let (_, hm) = super::HeightMap::<8, 5>::parse_height_map(&input).unwrap();
        let (_path, cost) = hm.find_path2().unwrap();
        assert_eq!(cost, 31);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("test_input/day12.txt").unwrap();
        let (_, hm) = super::HeightMap::<8, 5>::parse_height_map(&input).unwrap();
        let length = hm.find_shortest_path().unwrap();
        assert_eq!(length, 29);
    }
}
