use std::collections::{HashMap, HashSet};

pub type InputType = Vec<RucksackSet>;
pub type OutputType = usize;

pub struct Rucksack {
    //I thought I would need to count the duplicates, I guess not. I could have Just done sets the
    //entire time
    first: HashMap<char, usize>,
    second: HashMap<char, usize>,
}

//Implement the Set only version that doesn't care about counts to see performance enhancements
pub struct RucksackSet {
    first: HashSet<char>,
    second: HashSet<char>,
}

// Rucksack bench
// Day3 - Part1/(default)  time:   [112.59 us 112.70 us 112.81 us]
// Day3 - Part2/(default)  time:   [258.37 us 258.73 us 259.19 us]
// Generator               time:   [255.17 us 255.97 us 256.98 us]
//
// RucksackSet
// Day3 - Part1/(default)  time:   [25.117 us 25.159 us 25.201 us]
// Day3 - Part2/(default)  time:   [176.10 us 176.28 us 176.47 us]
// Generator               time:   [194.21 us 194.63 us 195.05 us]

pub trait Sack
where
    Self: Sized,
{
    fn new(ln: &str) -> Self;
    fn find_duplicates(&self) -> Vec<char>;
    fn get_combined_set(&self) -> HashSet<char>;
    fn find_item_in_three(&self, second: &Self, third: &Self) -> Option<char>;
}

impl Sack for RucksackSet {
    fn new(ln: &str) -> Self {
        let len = ln.chars().count();
        assert_eq!(len % 2, 0);
        let mut cs = ln.chars();
        let first = cs.by_ref().take(len / 2).collect::<HashSet<char>>();
        let second = cs.by_ref().take(len / 2).collect::<HashSet<char>>();
        RucksackSet { first, second }
    }
    fn find_duplicates(&self) -> Vec<char> {
        self.first.intersection(&self.second).map(|c| *c).collect()
    }

    fn get_combined_set(&self) -> HashSet<char> {
        self.first.union(&self.second).map(|c| *c).collect()
    }

    fn find_item_in_three(&self, second: &Self, third: &Self) -> Option<char> {
        let f = self
            .get_combined_set()
            .intersection(&second.get_combined_set())
            .map(|c| *c)
            .collect::<HashSet<char>>();
        let final_set = f
            .intersection(&third.get_combined_set())
            .map(|c| *c)
            .collect::<Vec<_>>();
        assert!(final_set.iter().count() <= 1);
        final_set.iter().next().copied()
    }
}

impl Sack for Rucksack {
    fn new(ln: &str) -> Self {
        let len = ln.chars().count();
        assert_eq!(len % 2, 0);
        let mut cs = ln.chars();

        let mut first = HashMap::new();
        let mut second = HashMap::new();

        cs.by_ref()
            .take(len / 2)
            .for_each(|c| *first.entry(c).or_insert(0) += 1);
        cs.for_each(|c| *second.entry(c).or_insert(0) += 1);

        //println!("{:?} -- {:?}", first, second);

        Rucksack { first, second }
    }

    fn find_duplicates(&self) -> Vec<char> {
        let first_set = self
            .first
            .iter()
            .map(|(k, _)| *k)
            .collect::<HashSet<char>>();
        let second_set = self
            .second
            .iter()
            .map(|(k, _)| *k)
            .collect::<HashSet<char>>();
        first_set.intersection(&second_set).map(|c| *c).collect()
    }

    fn get_combined_set(&self) -> HashSet<char> {
        let first_set = self
            .first
            .iter()
            .map(|(k, _)| *k)
            .collect::<HashSet<char>>();
        let second_set = self
            .second
            .iter()
            .map(|(k, _)| *k)
            .collect::<HashSet<char>>();
        first_set.union(&second_set).map(|c| *c).collect()
    }

    fn find_item_in_three(&self, second: &Self, third: &Self) -> Option<char> {
        let first_two = self
            .get_combined_set()
            .intersection(&second.get_combined_set())
            .map(|c| *c)
            .collect::<HashSet<char>>();
        let final_set = first_two
            .intersection(&third.get_combined_set())
            .map(|c| *c)
            .collect::<Vec<_>>();
        assert!(final_set.iter().count() <= 1);

        final_set.iter().next().copied()
    }
}

fn convert_char_to_priority(c: &char) -> usize {
    //println!("{}",c);
    //I know, it's horrible
    if *c == '_' {
        return 0;
    }

    const LOWER_OFFSET: usize = 96;
    const HIGHER_OFFSET: usize = 38;
    let val = *c as usize;
    if val > LOWER_OFFSET {
        val - LOWER_OFFSET
    } else {
        val - HIGHER_OFFSET
    }
}

#[aoc_generator(day3)]
fn day3_parse(input: &str) -> InputType {
    input
        .split("\n")
        .map(|line| RucksackSet::new(line))
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &InputType) -> OutputType {
    part1_tmpl::<RucksackSet>(input)
}

pub fn part1_tmpl<T: Sack>(input: &InputType) -> OutputType {
    input
        .iter()
        .map(|r| {
            r.find_duplicates()
                .iter()
                .map(|c| convert_char_to_priority(c))
                .sum::<usize>()
        })
        .sum::<usize>()
}

#[aoc(day3, part2)]
pub fn part2(input: &InputType) -> OutputType {
    part2_tmpl::<RucksackSet>(input)
}

pub fn part2_tmpl<T: Sack>(input: &Vec<T>) -> OutputType {
    input
        .chunks(3)
        .map(|three_rucks| match &three_rucks {
            &[a, b, c] => convert_char_to_priority(&a.find_item_in_three(&b, &c).unwrap_or('_')),
            _ => panic!("Can this even happen?"),
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
    }

    #[test]
    fn day3_parse_test() {
        let ruck = Rucksack::new("aabbCa");
        assert_eq!(ruck.first[&'a'], 2);
        assert_eq!(ruck.first[&'b'], 1);
        assert_eq!(ruck.second[&'b'], 1);
        assert_eq!(ruck.second[&'C'], 1);
        assert_eq!(ruck.second[&'a'], 1);
    }

    #[test]
    fn day3_part1() {
        assert_eq!(part1(&day3_parse(get_test_input())), 157);
    }

    #[test]
    fn day3_part2() {
        assert_eq!(part2(&day3_parse(get_test_input())), 70);
    }
}
