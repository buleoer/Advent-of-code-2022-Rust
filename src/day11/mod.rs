use std::path::Path;
use std::fs::read_to_string;
use std::cell::RefCell;


const TURNS: u32 = 20;

#[derive(Debug, PartialEq)]
enum Operator {
    Plus,
    Times,
}

impl Operator {
    fn apply(&self, left: u32, right: u32) -> u32 {
        match self {
            Self::Plus => left + right,
            Self::Times => left * right,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Operand {
    Old,
    Value(u32),
}

impl Operand {
    fn get_value(&self, old: u32) -> u32 {
        match self {
            Self::Old => old,
            Self::Value(v) => *v,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Operation {
    op: Operator,
    left: Operand,
    right: Operand,
}


impl Operation {
    fn execute(&self, old: u32) -> u32 {
        self.op.apply(
            self.left.get_value(old),
            self.right.get_value(old)
        )
    }
}


#[derive(Debug, PartialEq)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    test_divisible: u32,
    monkey_if_true: u32,
    monkey_if_false: u32,
    items_inspected: u32,
}

impl Monkey {
    fn do_turn(&mut self, monkeys: &MonkeyGroup) {
        for item in &self.items {

            // inspect item
            self.items_inspected += 1;

            // update worry level of the item
            let mut new_worry_level: u32 = self.operation.execute(*item);

            // update worry level after inspection (divide by 3)
            new_worry_level = (new_worry_level as f32 / 3.0).floor() as u32;

            // throw item to other monkey
            if new_worry_level % self.test_divisible == 0 {
                monkeys[self.monkey_if_true as usize].borrow_mut().items.push(new_worry_level);
            } else {
                monkeys[self.monkey_if_false as usize].borrow_mut().items.push(new_worry_level);
            }
        }
        self.items.clear();
    }
}

type MonkeyGroup = Vec<RefCell<Monkey>>;


mod parser{
    
    use std::cell::RefCell;

    use nom::IResult;
    use nom::error::ParseError;
    use nom::character::complete::{u32, char, multispace0, line_ending};
    use nom::bytes::complete::tag;
    use nom::sequence::{preceded, delimited, tuple};
    use nom::multi::{separated_list1, many1};
    use nom::branch::alt;
    use nom::combinator::map;

    type Operator = super::Operator;
    type Operand = super::Operand;
    type Operation = super::Operation;
    type Monkey = super::Monkey;
    type MonkeyGroup = super::MonkeyGroup;

    /// A combinator that takes a parser `inner` and produces a parser that also consumes leading
    /// whitespace, returning the output of `inner`.
    fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
        where
        F: FnMut(&'a str) -> IResult<&'a str, O, E>,
    {
        preceded(
            multispace0,
            inner
        )
    }

    /// A combinator that takes a parser `inner` and produces a parser that also consumes one leading
    /// carriage return, returning the output of `inner`.
    fn cr<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
        where
        F: FnMut(&'a str) -> IResult<&'a str, O, E>,
    {
        preceded(
            line_ending,
            inner
        )
    }

    fn parse_monkey_id(input: &str) -> IResult<&str, u32> {
        delimited(
            tag("Monkey "),
            u32,
            char(':')
        )(input)
    }

    fn parse_starting_items(input: &str) -> IResult<&str, Vec<u32>> {
        preceded(
            tag("  Starting items: "),
            separated_list1(
                tag(", "),
                u32
            )
        )(input)
    }

    fn parse_operand(input: &str) -> IResult<&str, Operand> {
        alt((
            map(tag("old"), |_| Operand::Old),
            map(u32, |v| Operand::Value(v)),
        ))(input)
    }

    fn parse_operator(input: &str) -> IResult<&str, Operator> {
        alt((
            map(char('+'), |_| Operator::Plus),
            map(char('*'), |_| Operator::Times),
        ))(input)
    }

    fn parse_operation(input: &str) -> IResult<&str, Operation> {
        map(
            tuple((
                tag("  Operation: new = "),
                ws(parse_operand),
                ws(parse_operator),
                ws(parse_operand),
            )),
            |(_, left, op, right)| Operation {op: op, left: left, right: right}
        )(input)
    }

    fn parse_test(input: &str) -> IResult<&str, u32> {
        preceded(
            tag("  Test: divisible by "),
            u32
        )(input)
    }

    fn parse_if_true(input: &str) -> IResult<&str, u32> {
        preceded(
            tag("    If true: throw to monkey "),
            u32
        )(input)
    }

    fn parse_if_false(input: &str) -> IResult<&str, u32> {
        preceded(
            tag("    If false: throw to monkey "),
            u32
        )(input)
    }

    fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
        map(
            tuple((
                parse_monkey_id,
                cr(parse_starting_items),
                cr(parse_operation),
                cr(parse_test),
                cr(parse_if_true),
                cr(parse_if_false),
            )),
            |(_, items, operation, test, if_true, if_false)| Monkey{
                items: items,
                operation: operation,
                test_divisible: test,
                monkey_if_true: if_true,
                monkey_if_false: if_false,
                items_inspected: 0,
            }
        )(input)
    }

    pub(super) fn parse_monkeys(input: &str) -> IResult<&str, MonkeyGroup> {
        separated_list1(
            many1(line_ending),
            map(parse_monkey, |monkey| RefCell::new(monkey))
        )(input)
    }

    #[cfg(test)]
    mod test {

        #[test]
        fn test_parse_monkey_id() {
            assert_eq!(
                super::parse_monkey_id("Monkey 23:"),
                Ok(("", 23))
            );
        }

        #[test]
        fn test_parse_starting_item() {
            assert_eq!(
                super::parse_starting_items("  Starting items: 2, 4, 10"),
                Ok(("", vec![2u32, 4, 10]))
            );
        }

        #[test]
        fn test_parse_operation() {
            assert_eq!(
                super::parse_operation("  Operation: new = old * 4"),
                Ok(("", super::Operation{
                    op: super::Operator::Times,
                    left: super::Operand::Old,
                    right: super::Operand::Value(4)
                }))
            );
        }

        #[test]
        fn test_parse_monkey() {
            assert_eq!(
                super::parse_monkey(r#"Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3"#),
                Ok(("", super::Monkey {
                    items: vec![79, 60, 97],
                    operation: super::Operation {
                        op: super::Operator::Times,
                        left: super::Operand::Old,
                        right: super::Operand::Old,
                    },
                    test_divisible: 13,
                    monkey_if_true: 1,
                    monkey_if_false: 3,
                    items_inspected: 0,
                }))
            )
        }
    }

}


fn find_most_active_monkeys(monkeys: &MonkeyGroup) -> (u32, u32) {
    let mut first: u32 = 0;
    let mut second: u32 = 0;
    for monkey in monkeys {
        let items_inspected = monkey.borrow().items_inspected;
        if items_inspected >= first {
            second = first;
            first = items_inspected;
        } else if items_inspected > second {
            second = items_inspected;
        }
    }
    (first, second)
}


pub fn run_part_1<P: AsRef<Path>>(filename: P) -> u32 {
    let input = read_to_string(filename).unwrap();
    let (_, monkeys) = parser::parse_monkeys(&input).unwrap();
    for _ in 0..TURNS {
        for monkey in &monkeys {
            monkey.borrow_mut().do_turn(&monkeys);
        }
    }
    let (first, second) = find_most_active_monkeys(&monkeys);
    first * second
}


pub fn run_part_2<P: AsRef<Path>>(_filename: P) -> i32 {
    0
}





#[cfg(test)]
mod test {

    #[test]
    fn test_part_1() {
        assert_eq!(super::run_part_1("test_input/day11.txt"), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::run_part_2("test_input/day11.txt"), 0);
    }

}

