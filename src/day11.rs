use std::cell::RefCell;
use std::collections::VecDeque;

type Item = usize;

#[derive(Debug, Clone)]
enum Operation {
    Plus(usize),
    Mult(usize),
    PlusSelf,
    MultSelf,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: VecDeque<Item>,
    true_throw: usize, //index of monkey
    false_throw: usize,
    test_div: usize,
    op: Operation,
    inspections: usize,
}

impl Operation {
    fn exec(&self, worry_level: Item) -> Item {
        match self {
            Operation::Plus(n) => worry_level + *n,
            Operation::Mult(n) => worry_level * *n,
            Operation::PlusSelf => worry_level + worry_level,
            Operation::MultSelf => worry_level * worry_level,
        }
    }
}

impl Monkey {
    fn new(lines: &str) -> Self {
        let mut monkey_lines = lines.split("\n");

        monkey_lines.next(); //Title

        let mut starting = monkey_lines.next().unwrap().split(" ");
        starting.next();
        starting.next();
        starting.next();
        starting.next();
        let starting_items: String = starting.collect::<String>();

        let items = starting_items
            .split(",")
            .map(|n| n.parse::<Item>().unwrap())
            .collect::<VecDeque<Item>>();

        //Operation
        let mut op_line = monkey_lines.next().unwrap().split(" ").skip(6);
        let op_sym = op_line.next().unwrap();
        let target = op_line.next().unwrap();

        let op = match (op_sym, target) {
            ("+", n) if n.parse::<i64>().is_ok() => Operation::Plus(n.parse::<usize>().unwrap()), //LAZY double parse
            ("*", n) if n.parse::<i64>().is_ok() => Operation::Mult(n.parse::<usize>().unwrap()),
            ("*", "old") => Operation::MultSelf,
            ("+", "old") => Operation::PlusSelf,
            _ => panic!("Bad parse"),
        };

        //Test
        let test_div = monkey_lines
            .next()
            .unwrap()
            .split(" ")
            .skip(5)
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        // if true
        let true_throw = monkey_lines
            .next()
            .unwrap()
            .split(" ")
            .skip(9)
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        // .next().unwrap().parse::<usize>().unwrap();
        let false_throw = monkey_lines
            .next()
            .unwrap()
            .split(" ")
            .skip(9)
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Monkey {
            items,
            true_throw,
            false_throw,
            test_div,
            op,
            inspections: 0,
        }
    }
}

type InputType = Vec<RefCell<Monkey>>;
type OutputType = usize;

#[aoc_generator(day11)]
fn day11_parse(input: &str) -> InputType {
    input
        .split("\n\n")
        .map(|lines| RefCell::new(Monkey::new(lines)))
        .collect::<Vec<_>>()
}

#[aoc(day11, part1)]
pub fn part1(input: &InputType) -> OutputType {
    monkey_business(input, false)
}

pub fn monkey_business(input: &InputType, part2: bool) -> OutputType {
    //AOC helper doesn't quite work if I tried to pass in a mut ref, input is small, so use
    //generator pattern but just clone it.
    let input = input.clone();

    let lcm: usize = input.iter().map(|m| m.borrow().test_div).product();

    let round_count: usize = if part2 { 10000 } else { 20 };

    //Lame to clone, but easier than wrapping everything in a RefCell or something
    for _round in 0..round_count {
        for (_monkey_idx, monkey_ref) in input.iter().enumerate() {
            let mut monkey = monkey_ref.borrow_mut();
            loop {
                let item = monkey.items.pop_front();
                if item.is_none() {
                    break;
                }
                monkey.inspections += 1;
                let cur_item = item.unwrap();

                //NOTE: Gross, since we can't have this get arbitrarily large, we need to scale
                //them but maintain their divisibility. To do this, we need to module the value by
                //its least common multiple (since the test values are all prime).
                // Blah, blah, blah math is hard.

                let new_worry_level = if part2 {
                    monkey.op.exec(cur_item) % lcm
                } else {
                    monkey.op.exec(cur_item) / 3
                };

                let next_monkey = if new_worry_level % monkey.test_div == 0 {
                    //divisible
                    monkey.true_throw
                } else {
                    monkey.false_throw
                };

                input
                    .get(next_monkey)
                    .unwrap()
                    .borrow_mut()
                    .items
                    .push_back(new_worry_level);
            }
        }
    }

    let mut inspections = input
        .iter()
        .map(|m| m.borrow().inspections)
        .collect::<Vec<_>>();
    inspections.sort();
    inspections.iter().rev().take(2).product()
}

#[aoc(day11, part2)]
pub fn part2(input: &InputType) -> OutputType {
    monkey_business(input, true)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
    }

    #[test]
    fn day11_part1() {
        assert_eq!(part1(&day11_parse(get_test_input())), 10605);
    }

    #[test]
    fn day11_part2() {
        assert_eq!(part2(&day11_parse(get_test_input())), 2713310158);
    }
}
