extern crate itertools;
use self::itertools::Itertools;
use std::cmp::{max, min};

use std::collections::HashMap;

type InputType = HashMap<(usize, usize), Space>;
type OutputType = usize;

#[derive(Clone)]
pub enum Space {
    Sand,
    Air,
    Rock,
    Source,
}

#[aoc_generator(day14)]
fn day14_parse(input: &str) -> InputType {
    let mut map = HashMap::new();
    input.split("\n").for_each(|line| {
        for (one, two) in line.split("->").tuple_windows() {
            let (x1, y1) = one
                .trim()
                .split(",")
                .map(|word| word.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            let (x2, y2) = two
                .trim()
                .split(",")
                .map(|word| word.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            //println!("({},{}) -> ({},{})", x1, y1, x2, y2);
            for x in min(x1, x2)..=max(x1, x2) {
                for y in min(y1, y2)..=max(y1, y2) {
                    map.insert((x, y), Space::Rock);
                }
            }
        }
    });

    map.insert((500, 0), Space::Source);

    map
}

fn print_debug_map(map: &InputType, min_x: usize, max_x: usize) {
    //Assume min_y is 0 and max_y is 10;
    let min_y = 0;
    let max_y = 10;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!(
                "{}",
                match map.get(&(x, y)).unwrap_or(&Space::Air) {
                    Space::Sand => "o",
                    Space::Air => ".",
                    Space::Rock => "#",
                    Space::Source => "+",
                }
            )
        }
        println!("");
    }
}

#[aoc(day14, part1)]
pub fn part1(input: &InputType) -> OutputType {
    simulate_physics(input, false)
}

pub fn simulate_physics(input: &InputType, part2: bool) -> OutputType {
    //yes, I'm copying the entire map, I know it's dumb, but the helper function does this, I could
    //just move the function and not use the generator but 🤷
    let mut map = HashMap::new();
    map.clone_from(input);

    //print_debug_map(&map, 494, 503);

    let max_seen_rocks = map
        .iter()
        .filter(|(_, v)| if let Space::Rock = v { true } else { false })
        .map(|((_, y), _)| *y)
        .max()
        .unwrap();

    let mut abyss = false;

    loop {
        //spawn sand
        let (mut s_x, mut s_y): (usize, usize) = (500, 0);
        loop {
            //Loop until we rest
            //check if down is open

            let (mut n_x, mut n_y) = (s_x, s_y);

            for (c_x, c_y) in [(s_x, s_y + 1), (s_x - 1, s_y + 1), (s_x + 1, s_y + 1)] {
                let mut stop = false;
                match map.get(&(c_x, c_y)).unwrap_or(&Space::Air) {
                    _ if c_y == max_seen_rocks + 2 && part2 => (),
                    Space::Sand | Space::Rock => (),
                    Space::Air | Space::Source => {
                        n_x = c_x;
                        n_y = c_y;
                        stop = true;
                    }
                };
                if stop {
                    break;
                }
            }

            //            println!("N ({},{})",n_x,n_y);

            if n_x == s_x && n_y == s_y {
                //Sand stopped moving
                break;
            }
            s_x = n_x;
            s_y = n_y;

            if n_y >= max_seen_rocks + 10 && !part2 {
                //Abyss check
                //Abyss
                abyss = true;
                break;
            }

            //println!("({},{}) {} max_seen_rocks = {}",n_x,n_y,max_seen_rocks,abyss);
        }
        //print_debug_map(&map, 494, 1000);
        if !part2 {
            if !abyss {
                //Don't insert final positions into the abyss
                map.insert((s_x, s_y), Space::Sand);
            } else {
                break;
            }
        } else {
            map.insert((s_x, s_y), Space::Sand);
            if (s_x, s_y) == (500, 0) {
                break;
            }
        }
    }

    map.iter()
        .filter(|(_, s)| if let Space::Sand = s { true } else { false })
        .count()
}

#[aoc(day14, part2)]
pub fn part2(input: &InputType) -> OutputType {
    simulate_physics(input, true)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
    }

    #[test]
    fn day14_part1() {
        assert_eq!(part1(&day14_parse(get_test_input())), 24);
    }

    #[test]
    fn day14_part2() {
        assert_eq!(part2(&day14_parse(get_test_input())), 93);
    }
}
