
use std::path::Path;
use std::fs;
use std::collections::HashSet;


#[derive(Debug, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn get_distance(&self, other: &Self) -> u64 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }
}


#[derive(Debug, PartialEq)]
pub struct Sensor {
    position: Position,
    nearest_beacon: Position,
}

#[derive(Debug, PartialEq)]
pub struct Input {
    sensors: Vec<Sensor>,
}


mod range {
    use std::cmp::{min, max};

    type Range = (i64, i64);

    pub const EMPTY: (i64, i64) = (0, -1);

    pub fn overlaps(r1: &Range, r2: &Range) -> bool {
        // https://stackoverflow.com/questions/3269434/whats-the-most-efficient-way-to-test-if-two-ranges-overlap
        (r1.0 <= r2.1) && (r2.0 <= r1.1)
    }

    pub fn merge(dest: &mut Range, source: &Range) {
        dest.0 = min(dest.0, source.0);
        dest.1 = max(dest.1, source.1);
    }

    pub fn get_size(r: &Range) -> u64 {
        (r.1 - r.0 + 1) as u64
    }

    #[cfg(test)]
    mod test {

        #[test]
        fn test_overlaps() {
            assert!(super::overlaps(&(1, 2), &(1, 2)));
            assert!(super::overlaps(&(1, 4), &(4, 8)));
            assert!(super::overlaps(&(10, 20), &(-10, 40)));
            assert!(super::overlaps(&(-10, 40), &(10, 20)));

            assert!(!super::overlaps(&(1, 4), &(5, 8)));
            assert!(!super::overlaps(&(-10, -44), &(4, 8)));
        }

        #[test]
        fn test_merge() {
            todo!();
        }
    }

}


impl Input {

    pub fn from_file<P: AsRef<Path>>(filename: P) -> Self {
        let input_string = fs::read_to_string(filename).unwrap();
        let (_, input) = parser::parse_input(&input_string).unwrap();
        input
    }

    pub fn run_part_1(&self, y_check: i64) -> u64 {
        let mut ranges: Vec<(i64, i64)> = Vec::new();
        let mut beacons_in_checked_row: HashSet<i64> = HashSet::new();
        
        for sensor in &self.sensors {
            if let Some(range) = Self::get_range_in_row_x(&sensor.position, &sensor.nearest_beacon, y_check) {
                ranges.push(range);
                let beacon_position = &sensor.nearest_beacon;
                if beacon_position.y == y_check {
                    beacons_in_checked_row.insert(beacon_position.x);
                }
            }
        }

        // sort ranges by starting position
        ranges.sort_by( |r1, r2| r1.0.cmp(&r2.0) );

        // Merge ranges when they overlap and count the total size
        let mut current_range = range::EMPTY;
        let positions_with_no_beacon = ranges.iter().fold(0u64, |acc, r| {
            if current_range == range::EMPTY {
                current_range = *r;
                return range::get_size(r);
            }
            if range::overlaps(&current_range, r) {
                let size_before = range::get_size(&current_range);
                range::merge(&mut current_range, &r);
                return acc - size_before + range::get_size(&current_range);
            }
            current_range = *r;
            acc + range::get_size(r)
        });

        positions_with_no_beacon - beacons_in_checked_row.len() as u64
    }

    fn get_range_in_row_x(sensor_position: &Position, beacon_position: &Position, row_of_range: i64) -> Option<(i64, i64)> {
        //distance between sensor and beacon
        let distance = sensor_position.get_distance(beacon_position);

        // vertical distance between the sensor and the row to check
        let diff = sensor_position.y.abs_diff(row_of_range);

        // let diff = (distance - (sensor_position.y - row_of_range).unsigned_abs()) as i64;
        if diff > distance {
            None
        }
        else {
            let diff = i64::try_from(distance).unwrap() - i64::try_from(diff).unwrap();
            Some((sensor_position.x - diff, sensor_position.x + diff))
        }
    }
}




mod parser {

    use super::{Position, Sensor, Input};

    use nom::IResult;
    use nom::bytes::complete::tag;
    use nom::character::complete::{i64, newline};
    use nom::sequence::{preceded, terminated, separated_pair, tuple};
    use nom::multi::many1;
    use nom::combinator::map;

    fn parse_x(input: &str) -> IResult<&str, i64> {
        preceded(
            tag("x="),
            i64
        )(input)
    }

    fn parse_y(input: &str) -> IResult<&str, i64> {
        preceded(
            tag("y="),
            i64
        )(input)
    }

    fn parse_position(input: &str) -> IResult<&str, Position> {
        map(
            separated_pair(
                parse_x,
                tag(", "),
                parse_y
            ),
            |(x, y)| Position{ x, y }
        )(input)
    }

    fn parse_sensor(input: &str) -> IResult<&str, Sensor> {
        map(
            tuple((
                tag("Sensor at "),
                parse_position,
                tag(": closest beacon is at "),
                parse_position
            )),
            |(_, pos1, _, pos2)| Sensor{ position: pos1, nearest_beacon: pos2 }
        )(input)
    }

    pub fn parse_input(input: &str) -> IResult<&str, Input> {
        map(
            many1(
                terminated(
                    parse_sensor,
                    newline
                )
            ),
            |v| Input{sensors: v}
        )(input)
    }


    #[cfg(test)]
    mod test {

        use super::{Position, Sensor, Input};

        #[test]
        fn test_parse_x_y() {
            assert_eq!(super::parse_x("x=20"), Ok(("", 20i64)));
            assert_eq!(super::parse_y("y=-9"), Ok(("", -9i64)));
        }

        #[test]
        fn test_parse_position() {
            assert_eq!(
                super::parse_position("x=20, y=-9"),
                Ok(("", Position{x: 20, y: -9}))
            );
        }

        #[test]
        fn test_parse_sensor() {
            assert_eq!(
                super::parse_sensor("Sensor at x=16, y=7: closest beacon is at x=15, y=3"),
                Ok(("", 
                    Sensor{
                        position: Position {x: 16, y: 7 },
                        nearest_beacon: Position{ x: 15, y: 3 }
                    }
                ))
            );
        }

        #[test]
        fn test_parse_input() {
            assert_eq!(
                super::parse_input(concat!(
                    "Sensor at x=16, y=7: closest beacon is at x=15, y=3\n",
                    "Sensor at x=-3, y=12: closest beacon is at x=7, y=-13\n"
                )),
                Ok(("",
                    Input{
                        sensors: vec![
                            Sensor{
                                position: Position{ x: 16, y: 7 },
                                nearest_beacon: Position{ x: 15, y: 3 }
                            },
                            Sensor{
                                position: Position{ x: -3, y: 12 },
                                nearest_beacon: Position{ x: 7, y: -13 }
                            },
                        ]
                    }
                ))
            );
        }
    }
}


pub fn run_part_1<P: AsRef<Path>>(filename: P) -> u64 {
    let input = Input::from_file(filename);
    input.run_part_1(2000000)
}

pub fn run_part_2<P: AsRef<Path>>(_filename: P) -> u32 {
    0
}






#[cfg(test)]
mod test {

    use super::{Input, Position};

    #[test]
    fn test_distance_calculation() {
        assert_eq!(Position{ x: 1, y: 1}.get_distance(&Position{ x: 3, y: 3}), 4);
        assert_eq!(Position{ x: -2, y: -2}.get_distance(&Position{ x: 2, y: 2}), 8);
        assert_eq!(Position{ x: 8, y: 9}.get_distance(&Position{ x: 8, y: 9}), 0);
    }

    #[test]
    fn test_range_calculation() {
        assert_eq!(
            Input::get_range_in_row_x(
                &Position{x: 31, y: 19},
                &Position{x: 36, y: 15},
                26
            ),
            Some((29, 33))
        )
    }

    #[test]
    fn test_part_1() {
        let input = Input::from_file("test_input/day15.txt");
        assert_eq!(input.run_part_1(10), 26);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(0, 1);
    }

}


