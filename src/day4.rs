extern crate lazy_static;
//#[macro_use] extern crate lazy_static;
use self::lazy_static::lazy_static;
extern crate regex;
use self::regex::Regex;

pub struct ElfPair {
    elf_1_lower: u64,
    elf_1_higher: u64,
    elf_2_lower: u64,
    elf_2_higher: u64,
}

type InputType = Vec<ElfPair>;
type OutputType = u64;

impl ElfPair {
    fn new(inp: &str) -> Self {
        //105 us; ~2.43 speedup
        let mut chars = inp.chars();
        let elf_1_lower = chars
            .by_ref()
            .take_while(|&c| c != '-')
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let elf_1_higher = chars
            .by_ref()
            .take_while(|&c| c != ',')
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let elf_2_lower = chars
            .by_ref()
            .take_while(|&c| c != '-')
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let elf_2_higher = chars.collect::<String>().parse::<u64>().unwrap();

        ElfPair {
            elf_1_lower,
            elf_1_higher,
            elf_2_lower,
            elf_2_higher,
        }
    }

    fn new_regex(inp: &str) -> Self {
        // 255 us
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
        }
        //Regex is easier, probably more computationally efficient to chars().take_until()
        let caps = RE.captures(inp).unwrap(); //panic if malformed

        //We already pulled them out with a
        //numeric regex, we can assume they
        //are correct
        ElfPair {
            elf_1_lower: caps[1].parse::<u64>().unwrap(),
            elf_1_higher: caps[2].parse::<u64>().unwrap(),
            elf_2_lower: caps[3].parse::<u64>().unwrap(),
            elf_2_higher: caps[4].parse::<u64>().unwrap(),
        }
    }

    //395 too low
    fn one_in_pair_is_subset(&self) -> bool {
        assert!(self.elf_1_lower <= self.elf_1_higher);
        assert!(self.elf_2_lower <= self.elf_2_higher);

        if self.elf_1_lower < self.elf_2_lower {
            //The first elf is "lower" the second must be within it
            // [X1             X2]
            //    [Y1 ...      Y2]
            if self.elf_1_lower <= self.elf_2_higher && self.elf_1_higher >= self.elf_2_higher {
                return true;
            }
        } else if self.elf_1_lower == self.elf_2_lower {
            //If they're both the same start, then either one is certainly a subset of the other
            return true;
        } else {
            // elf 2 is the lower bound
            if self.elf_2_lower <= self.elf_1_higher && self.elf_2_higher >= self.elf_1_higher {
                return true;
            }
        }
        //println!("{}-{},{}-{}",self.elf_1_lower, self.elf_1_higher, self.elf_2_lower, self.elf_2_higher);
        return false;
    }

    fn any_overlap(&self) -> bool {
        if self.elf_1_lower < self.elf_2_lower {
            // If x_1 is lower than y_1, then it's a subset as long as x_1 does not begin before
            // y_1
            if self.elf_1_higher >= self.elf_2_lower {
                return true;
            }
        } else if self.elf_1_lower == self.elf_2_lower {
            //If they start at the same place, they of course, MUST overlap
            //I don't check if the higher is the same, because all the other comparisions are on
            //the lower bound, and would get captured by the other checks
            return true;
        } else {
            //Same here but the opposite
            if self.elf_2_higher >= self.elf_1_lower {
                return true;
            }
        }
        return false;
    }
}

#[aoc_generator(day4)]
fn day4_parse(input: &str) -> InputType {
    input.split("\n").map(|line| ElfPair::new(line)).collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &InputType) -> OutputType {
    input
        .iter()
        .filter(|pair| pair.one_in_pair_is_subset())
        .count() as u64
}

#[aoc(day4, part2)]
pub fn part2(input: &InputType) -> OutputType {
    input.iter().filter(|pair| pair.any_overlap()).count() as u64
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(&day4_parse(get_test_input())), 2);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&day4_parse(get_test_input())), 4);
    }
}
