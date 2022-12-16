
extern crate lazy_static;
use self::lazy_static::lazy_static;
extern crate regex;
use self::regex::Regex;

use std::collections::{HashMap,HashSet,VecDeque};


pub struct Node {
    label: String,
    rate: i32,
    links: Vec<String> //implicit links
}

type InputType = HashMap<String,Node>;
type OutputType = usize;

#[aoc_generator(day16)]
fn day16_parse(input: &str) -> InputType {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]*)"
        )
        .unwrap();
    }

    let mut tree = HashMap::new();

    for line in input.lines() {
        let caps = RE.captures(line).unwrap();
        let label = caps.get(1).unwrap().as_str();
        let rate = caps.get(2).unwrap().as_str().parse().unwrap();
        let neighbors = caps.get(3).unwrap().as_str().split(",").map(|x| x.trim().to_owned()).collect::<Vec<_>>();

        tree.insert(label.to_owned(), Node{ label: label.to_owned(), rate: rate, links: neighbors });
    }

    tree

}

fn dfs(input: &InputType, time: usize) -> OutputType {
    //Traverse the graph, but have arbitrary stop time (so that even with loops we stop
    let max_pressure = input.values().map(|n| n.rate).sum::<i32>();
    let mut accumulated_relief = 0;
    let mut cur_time = 0;
    let mut opened_valves : HashSet<String> = HashSet::new();

    let mut current_neighbors = VecDeque::new();
    for link in &input.get("AA").unwrap().links {
        current_neighbors.push_back(link);
    }
    //Initial setup






    accumulated_relief



}

#[aoc(day16, part1)]
pub fn part1(input: &InputType) -> OutputType {
    dfs(input,30)
}

#[aoc(day16, part2)]
pub fn part2(input: &InputType) -> OutputType {
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"
    }

    #[test]
    fn day16_part1() {
        assert_eq!(part1(&day16_parse(get_test_input())), 1651);
    }

    #[test]
    fn day16_part2() {
        assert_eq!(part2(&day16_parse(get_test_input())), 0);
    }
}
