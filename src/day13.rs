extern crate itertools;
use self::itertools::Itertools;
use std::cmp::{Ordering, PartialOrd};

use std::fmt;

#[derive(Debug, Clone)]
pub enum Packet {
    List(Vec<Packet>),
    Raw(usize),
}

type InputType = Vec<(Packet, Packet)>;
type OutputType = usize;

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Packet::List(vp) => write!(f, "[{}]", vp.iter().map(|p| format!("{}", p)).join(",")),
            Packet::Raw(n) => write!(f, "{}", n),
        }
    }
}

impl Packet {
    fn new(line: &str) -> Packet {
        let pack = Packet::new_helper(line, 0).unwrap();
        assert_eq!(line, format!("{}", pack));
        //println!("Packet: {:?}", pack);
        pack
    }

    fn find_packet_end_index(line: &str) -> usize {
        //println!("Finding enclosing brackets for {}", line);
        let mut depth = 1;
        for (idx, c) in line.chars().enumerate().skip(1) {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                _ => depth += 0,
            }
            if depth == 0 {
                //println!("Ending idx: {}", idx);
                return idx;
            }
        }
        panic!("No ending bracket")
    }

    //Return how many characters were consumed so we can skip (include parsing the bracket or any
    //comma
    fn new_helper(line: &str, depth: usize) -> Option<Packet> {
        let cit = line.chars().collect::<Vec<char>>();
        let max = line.chars().count();

        //println!("Parsing {:?} max: {}", line, max);

        // if cit.clone().len() == 0 {
        //     return None;
        // }
        let mut inner_packets: Vec<Packet> = vec![];
        let mut cur_offset = 0;

        while cur_offset < max {
            let next_letter_o = cit.get(cur_offset);
            if next_letter_o.is_none() {
                //println!("Break!");
                break;
            }
            let next_letter = *next_letter_o.unwrap();
            //println!("Next char '{}' offset: {}", next_letter, cur_offset);

            if next_letter == '[' {
                //println!("Saw start of a list, adding it");
                let end_idx = Self::find_packet_end_index(&line[cur_offset..]);
                if let Some(inner) =
                    Self::new_helper(&line[cur_offset + 1..end_idx + cur_offset], depth + 1)
                {
                    //inner_packets.push(inner);
                    //Uh... super lame
                    if depth == 0 {
                        match inner {
                            Packet::List(mut list) => inner_packets.append(&mut list),
                            Packet::Raw(_) => inner_packets.push(inner),
                        }
                    } else {
                        inner_packets.push(inner);
                    }
                }
                cur_offset += end_idx;
            } else if next_letter == ']' || next_letter == ',' {
                //println!("Saw {} adding 1", next_letter);
                cur_offset += 1;
            } else {
                //assume it's a number or the start of one, parse until we hit an endpoint (, or ]
                //or [)
                let num = line
                    .chars()
                    .skip(cur_offset)
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>();
                inner_packets.push(Packet::Raw(num.parse::<usize>().unwrap()));
                cur_offset += num.chars().count();
                // println!(
                //     "Saw number {} it was {} bytes/chars long new co {}",
                //     num,
                //     num.chars().count(),
                //     cur_offset
                // );
            }
        }

        return Some(Packet::List(inner_packets));
    }

    fn right_order(&self, other: &Self) -> Ternary {
        match (self, other) {
            (Packet::Raw(n1), Packet::Raw(n2)) => {
                if n1 < n2 {
                    return Ternary::Good;
                } else if n1 == n2 {
                    return Ternary::Cont;
                } else {
                    return Ternary::Bad;
                }
            }
            (Packet::List(l1), Packet::List(l2)) => {
                let l1_len = l1.iter().count();
                let l2_len = l2.iter().count();

                //Edge case, empty list
                if l1_len == 0 && l2_len > 0 {
                    return Ternary::Good;
                }
                if l1_len > 0 && l2_len == 0 {
                    return Ternary::Bad;
                }

                for (idx, (i1, i2)) in l1.iter().zip(l2.iter()).enumerate() {
                    //println!("Comparing {} {}", i1, i2);
                    match i1.right_order(i2) {
                        Ternary::Good => return Ternary::Good,
                        Ternary::Bad => return Ternary::Bad,
                        Ternary::Cont => (),
                    };
                    //Check if we are about to run out
                    if idx + 1 == l2_len && l2_len != l1_len {
                        //if right runs out first, bad
                        return Ternary::Bad;
                    } else if idx + 1 == l1_len && l2_len != l1_len {
                        //if left runs out first, good
                        return Ternary::Good;
                    }
                }
                // if we  made it all the way to here, both lists are the same size and had
                // matching numbers //continue checking?
                return Ternary::Cont;
            }
            //Just be lazy AF with the clones
            (Packet::List(_), Packet::Raw(_)) => {
                return self.right_order(&Packet::List(vec![other.clone()]))
            }
            (Packet::Raw(_), Packet::List(_)) => {
                return Packet::List(vec![self.clone()]).right_order(other)
            }
        }
    }
}
enum Ternary {
    Good,
    Bad,
    Cont,
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        format!("{}", self) == format!("{}", other)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Packet {}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.right_order(other) {
            Ternary::Good => Ordering::Less,
            Ternary::Bad => Ordering::Greater,
            Ternary::Cont => Ordering::Equal, //Should never show up; would imply an equal
                                              //packet
        }
    }
}

#[aoc_generator(day13)]
fn day13_parse(input: &str) -> InputType {
    input
        .split("\n\n")
        .map(|dubs| {
            dubs.split("\n")
                .map(|line| Packet::new(line))
                .collect_tuple::<(Packet, Packet)>()
                .unwrap()
        })
        .collect::<InputType>()
}

#[aoc(day13, part1)]
pub fn part1(input: &InputType) -> OutputType {
    input
        .iter()
        .enumerate()
        //.inspect(|(idx,(_,_))| println!("Looking at Index {}", idx +1))
        .map(|(idx, (p1, p2))| {
            if let Ternary::Good = p1.right_order(p2) {
                idx + 1
            } else {
                0
            }
        })
        //.inspect(|x| println!("Last Val {}", x))
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &InputType) -> OutputType {
    static MARKER_ONE: &str = "[[2]]";
    static MARKER_TWO: &str = "[[6]]";

    let packet_one = Packet::new(MARKER_ONE);
    let packet_two = Packet::new(MARKER_TWO);

    let mut all_packets = input
        .iter()
        .map(|(one, two)| [one, two])
        .flatten()
        .collect::<Vec<_>>();
    //inject marker packets
    all_packets.push(&packet_one);
    all_packets.push(&packet_two);

    //order
    all_packets.sort();

    all_packets
        .iter()
        .enumerate()
        .map(|(idx, pack)| {
            let string_rep = format!("{}", pack);
            if string_rep == MARKER_ONE || string_rep == MARKER_TWO {
                idx + 1
            } else {
                1
            }
        })
        .product()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_blank_input() -> &'static str {
        "[[[]]]
[[]]"
    }

    fn get_test_input() -> &'static str {
        "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
    }

    #[test]
    fn day13_parsing_is_hard() {
        let inp = "[[1],[2,3,4]]";
        let out = Packet::new(inp);
        //println!("In {} out {}", inp, out);
        assert_eq!(inp, format!("{}", out));
    }

    #[test]
    fn day13_part1() {
        assert_eq!(part1(&day13_parse(get_test_input())), 13);
    }

    // #[test]
    // fn day13_part1_blanks() {
    //     assert_eq!(part1(&day13_parse(get_blank_input())), 13);
    // }

    #[test]
    fn day13_part2() {
        assert_eq!(part2(&day13_parse(get_test_input())), 140);
    }
}
