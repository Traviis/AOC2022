use std::collections::VecDeque;
use std::usize;

pub type Crate = char;

pub struct Instruction {
    how_many: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn new(line: &str) -> Self {
        let mut out = line.split(" ");
        out.next().unwrap(); //move

        let how_many = out.next().unwrap().parse::<usize>().unwrap();

        out.next(); //from
        let from = out.next().unwrap().parse::<usize>().unwrap();

        out.next(); //to
        let to = out.next().unwrap().parse::<usize>().unwrap();

        Instruction { how_many, from, to }
    }
}

pub type Stack = Vec<Crate>;
type InputType = (Vec<Stack>, Vec<Instruction>);
type OutputType = String;

#[aoc_generator(day5)]
fn day5_parse(input: &str) -> InputType {
    //Cheat here, instead of doing a prescan pass and determining how many stacks we have, assume
    //we have no more than 19 (+1) for the intentionally left blank so I don't have to do index
    //math, and be on your way; if I wanted this to be more resiliant, I would of course, do
    //something fancier here (perhaps even just use a HashMap and map the values there)
    let mut stacks: Vec<Stack> = Vec::new();
    for _ in 0..20 {
        stacks.push(vec![]);
    }

    let mut section_iter = input.split("\n\n");
    let stacks_iter = section_iter.next().unwrap();
    stacks_iter.split("\n").for_each(|line| {
        line.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .enumerate()
            .for_each(|(stack_idx, qgram)| {
                let qgram = qgram.iter().collect::<String>();
                if qgram.contains("[") {
                    let mut crate_chars = qgram.chars();
                    crate_chars.next();
                    stacks[stack_idx + 1 as usize].push(crate_chars.next().unwrap())
                }
            })
    });
    //Flip them over, could have used vecdeque but that doens't implement chunkable, these are tiny
    //enough I don't care
    let stacks = stacks
        .iter()
        .map(|stack| stack.into_iter().rev().map(|c| *c).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    //println!("{:?}", stacks);

    let instructions = section_iter
        .next()
        .unwrap()
        .split("\n")
        .map(|line| Instruction::new(line))
        .collect();

    (stacks, instructions)
}

#[aoc(day5, part1)]
pub fn part1((stacks, instructions): &InputType) -> OutputType {
    let mut stacks = stacks.clone();
    for inst in instructions {
        let Instruction { how_many, from, to } = inst;
        for _ in 0..*how_many {
            if let Some(val) = stacks[*from].pop() {
                stacks[*to].push(val);
                // println!("Moving from {} to {} = {}",from,to,val);
                // println!("{:?}",stacks);
            }
        }
    }
    //println!("{:?}",stacks);
    stacks
        .iter()
        .filter_map(|stack| stack.last())
        .collect::<String>()
}

#[aoc(day5, part2)]
pub fn part2((stacks, instructions): &InputType) -> OutputType {
    let mut stacks = stacks.clone();
    for inst in instructions {
        let Instruction { how_many, from, to } = inst;
        let mut handle = VecDeque::new();
        for _ in 0..*how_many {
            if let Some(val) = stacks[*from].pop() {
                handle.push_front(val);
                // println!("Moving from {} to {} = {}",from,to,val);
                // println!("{:?}",stacks);
            }
        }
        for cr in handle.into_iter() {
            stacks[*to].push(cr);
        }
    }

    stacks
        .iter()
        .filter_map(|stack| stack.last())
        .collect::<String>()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "    [D]
[N] [C]
[Z] [M] [P]
1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
    }

    #[test]
    fn day5_part1() {
        assert_eq!(part1(&day5_parse(get_test_input())), "CMZ");
    }

    #[test]
    fn day5_part2() {
        assert_eq!(part2(&day5_parse(get_test_input())), "MCD");
    }
}
