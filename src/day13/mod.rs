use std::path::Path;
use std::fs;
use std::cmp::Ordering;



#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Integer(u32),
    List(Box<Vec<Self>>),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Integer(v_self), Value::Integer(v_other)) => v_self.cmp(v_other),
            (Value::List(v_self), Value::List(v_other)) => {
                for (v_self, v_other) in (*v_self).iter().zip((*v_other).iter()) {
                    let o = v_self.cmp(v_other);
                    if o != Ordering::Equal { return o; }
                }
                // if all elements equals, shortest list first
                v_self.len().cmp(&v_other.len())
            },
            (Value::List(_), Value::Integer(v_other)) => {
                let l_other = Value::List(Box::new(vec!(Value::Integer(*v_other))));
                self.cmp(&l_other)
            },
            (Value::Integer(v_self), Value::List(_)) => {
                let l_self = Value::List(Box::new(vec!(Value::Integer(*v_self))));
                l_self.cmp(other)
            },
        }
    }
}


impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug, PartialEq)]
pub struct PacketPair {
    p1: Value,
    p2: Value,
}

impl PacketPair {
    pub fn is_in_right_order(&self) -> bool {
        self.p1 <= self.p2
    }
}


pub struct Signal (Vec<PacketPair>);


impl Signal {

    pub fn from_file<P: AsRef<Path>>(filename: P) -> Self {
        let input = fs::read_to_string(filename).unwrap();
        let (_, signal) = parser::parse_signal(&input).unwrap();
        signal
    }

    pub fn count_pairs_in_right_order(&self) -> u32 {
        self.0.iter().enumerate().fold(0u32, |sum, (i, pp)| {
            if pp.is_in_right_order() {
                sum + i as u32 + 1
            } else {
                sum
            }
        })
    }

    pub fn calculate_part_2(&self) -> u32 {
        let mut v: Vec<&Value> = Vec::new();
        for pp in &self.0 {
            v.push(&pp.p1);
            v.push(&pp.p2);
        }
        let marker1 = &Value::List(Box::new(vec!(
            Value::List(Box::new(vec!(
                Value::Integer(2)
            )))
        )));
        v.push(&marker1);
        let marker2 = &Value::List(Box::new(vec!(
            Value::List(Box::new(vec!(
                Value::Integer(6)
            )))
        )));
        v.push(&marker2);

        v.sort();

        let mut result = 1u32;
        for (i, val) in v.iter().enumerate() {
            if (val == &marker1) || (val == &marker2) {
                result *= (i+1) as u32;
            }
        }
        result
    }
}



mod parser {
    use super::{Value, PacketPair, Signal};

    use nom::IResult;
    use nom::character::complete::{u32, char, line_ending, multispace1};
    use nom::multi::{separated_list0, separated_list1};
    use nom::branch::alt;
    use nom::sequence::{delimited, separated_pair};
    use nom::combinator::map;

    fn parse_value(input: &str) -> IResult<&str, Value> {
        alt((
            map(u32, |v| Value::Integer(v)),
            parse_list,
        ))(input)
    }

    fn parse_list(input: &str) -> IResult<&str, Value> {
        delimited(
            char('['),
            map(
                separated_list0(char(','), parse_value),
                |v| Value::List(Box::new(v)),
            ),
            char(']')
        )(input)
    }

    fn parse_packet_pair(input: &str) -> IResult<&str, PacketPair> {
        map(
            separated_pair(
                parse_list,
                line_ending,
                parse_list
            ),
            |(p1, p2)| PacketPair{ p1, p2 }
        )(input)
    }

    pub fn parse_signal(input: &str) -> IResult<&str, Signal> {
        map(
            separated_list1(
                multispace1,
                parse_packet_pair
            ),
            |v| Signal(v)
        )(input)
    }

    #[cfg(test)]
    mod test {

        use super::Value;

        #[test]
        fn test_parse_value() {
            assert_eq!(
                super::parse_value("12"),
                Ok(("", Value::Integer(12)))
            );

            assert_eq!(
                super::parse_value("[]"),
                Ok(("", Value::List(Box::new(vec!()))))
            );

            assert_eq!(
                super::parse_value("[1,2,3,[10,20,0]]"),
                Ok(("", Value::List(Box::new(
                    vec!(
                        Value::Integer(1),
                        Value::Integer(2),
                        Value::Integer(3),
                        Value::List(Box::new(
                            vec!(
                                Value::Integer(10),
                                Value::Integer(20),
                                Value::Integer(0)
                            )
                        ))
                    )
                ))))
            );

        }
    }    
}



pub fn run_part_1<P: AsRef<Path>>(filename: P) -> u32 {
    let signal = Signal::from_file(filename);
    signal.count_pairs_in_right_order()
}


pub fn run_part_2<P: AsRef<Path>>(filename: P) -> u32 {
    let signal = Signal::from_file(filename);
    signal.calculate_part_2()
}


#[cfg(test)]
mod test {

    #[test]
    fn test_part_1() {
        let signal = super::Signal::from_file("test_input/day13.txt");
        assert_eq!(signal.count_pairs_in_right_order(), 13);
    }

    #[test]
    fn test_part_2() {
        let signal = super::Signal::from_file("test_input/day13.txt");
        assert_eq!(signal.calculate_part_2(), 140);
    }

}

