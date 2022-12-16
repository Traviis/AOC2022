extern crate lazy_static;
use self::lazy_static::lazy_static;
extern crate regex;
use self::regex::Regex;
use std::collections::{HashMap, HashSet};
extern crate rayon;
use self::rayon::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;

use std::cmp;

#[derive(Debug)]
pub enum Item {
    Sensor,
    Beacon,
    Nothing,
}

type InputType = (
    HashMap<(i128, i128), Item>,
    HashMap<(i128, i128), (i128, i128)>,
);
type OutputType = i128;

#[aoc_generator(day15)]
fn day15_parse(input: &str) -> InputType {
    let mut map = HashMap::new();
    let mut closest_beacons = HashMap::new();

    for line in input.split("\n") {
        //println!("Looking at line: {}",line);
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Sensor at x=([0-9-]+), y=([0-9-]+): closest beacon is at x=([0-9-]+), y=([0-9-]+)"
            )
            .unwrap();
        }
        let caps = RE.captures(line).unwrap();
        let mut caps = caps
            .iter()
            .skip(1)
            .map(|c| c.unwrap().as_str().parse::<i128>().unwrap());

        let s_x = caps.next().unwrap();
        let s_y = caps.next().unwrap();
        let b_x = caps.next().unwrap();
        let b_y = caps.next().unwrap();

        map.insert((s_x, s_y), Item::Sensor);
        map.insert((b_x, b_y), Item::Beacon);
        closest_beacons.insert((s_x, s_y), (b_x, b_y));
    }

    (map, closest_beacons)
}

fn print_debug_map(
    map: &HashMap<(i128, i128), Item>,
    max_x: usize,
    max_y: usize,
    sens_range: &HashSet<(i128, i128)>,
) {
    for y in -2..=max_y as i128 {
        for x in -2..=max_x as i128 {
            print!(
                "{}",
                match map.get(&(x as i128, y as i128)).unwrap_or(&Item::Nothing) {
                    Item::Sensor => "S",
                    Item::Beacon => "B",
                    Item::Nothing =>
                        if sens_range.get(&(x as i128, y as i128)).is_some() {
                            "#"
                        } else {
                            "."
                        },
                }
            );
        }
        println!("");
    }
}

fn manhatten((x1, y1): (i128, i128), (x2, y2): (i128, i128)) -> i128 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn bounded_naive_determine_sensor_range(
    (s_x, s_y): (i128, i128),
    (b_x, b_y): (i128, i128),
    min_x: i128,
    max_x: i128,
    min_y: i128,
    max_y: i128,
) -> HashSet<(i128, i128)> {
    let dist = manhatten((s_x, s_y), (b_x, b_y));

    //We only care about a very specific y coordinate

    //TODO: This can be optimized more, but let's just try to brute force it with checking the row
    let mut out = HashSet::new();
    out.insert((s_x,s_y));
    //for m_y in s_y - dist..=s_y + dist {
    for m_y in min_y..=max_y {
        for m_x in s_x - dist..=s_x + dist {
            if manhatten((m_x, m_y), (s_x, s_y)) <= dist {
                    out.insert((m_x, m_y));
            }
        }
    }
    out
}
fn naive_determine_sensor_range(
    (s_x, s_y): (i128, i128),
    (b_x, b_y): (i128, i128),
    cared_y: i128,
) -> HashSet<(i128, i128)> {
    //There is probably a clever way of calculating this, but brute force this bad boy!
    bounded_naive_determine_sensor_range(
        (s_x, s_y),
        (b_x, b_y),
        i128::MIN,
        i128::MAX,
        cared_y,
        cared_y,
    )
}
fn determine_sensor_range(
    (s_x, s_y): (i128, i128),
    (b_x, b_y): (i128, i128),
) -> HashSet<(i128, i128)> {
    let dist = manhatten((s_x, s_y), (b_x, b_y));
    todo!();
}

#[aoc(day15, part1)]
pub fn part1(inp: &InputType) -> OutputType {
    part1_param(inp, 2000000)
}

//TODO: Ok, the slam it together approach sucks, I should rewrite this to not dump everything into
//a set, but instead just write a function that will determine if a given point is within sensor
//range.

pub fn part1_param((map, closest_map): &InputType, cared_y: i128) -> OutputType {
    let mut all_ranges = Arc::new(Mutex::new(HashSet::new()));

    closest_map
        .par_iter()
        .map(|((s_x, s_y), (b_x, b_y))| {
            naive_determine_sensor_range((*s_x, *s_y), (*b_x, *b_y), cared_y)
        })
        .for_each(|h_set| {
            let ranges = Arc::clone(&all_ranges);
            let mut new_set = ranges.lock().unwrap();
            *new_set = (*new_set)
                .union(&h_set)
                .map(|x| *x)
                .collect::<HashSet<(i128, i128)>>();
        });


    let cant_be = all_ranges.lock().unwrap();

   cant_be 
        .iter()
        .filter(|(_, y)| *y == cared_y)
        .filter(|(x, y)| map.get(&(*x, *y)).is_none())
        .count() as i128
}

#[aoc(day15, part2)]
pub fn part2(inp: &InputType) -> OutputType {
    part2_param(inp, 2000000)
}
pub fn part2_param((map, closest_map): &InputType, max_v: i128) -> OutputType {
    let mut g_x = 0;
    let mut g_y = 0;

    for c_y in 0..max_v {
        let mut all_ranges = Arc::new(Mutex::new(HashSet::new()));
        closest_map
            .par_iter()
            .map(|((s_x, s_y), (b_x, b_y))| {
                bounded_naive_determine_sensor_range((*s_x, *s_y), (*b_x, *b_y), 0, max_v, c_y, c_y)
                //naive_determine_sensor_range((*s_x, *s_y), (*b_x, *b_y),  c_y)
            })
            .for_each(|h_set| {
                let ranges = Arc::clone(&all_ranges);
                let mut new_set = ranges.lock().unwrap();
                *new_set = (*new_set)
                    .union(&h_set)
                    .map(|x| *x)
                    .collect::<HashSet<(i128, i128)>>();
            });

        let cant_be = all_ranges.lock().unwrap();

        let cant_be_here = cant_be
            .iter()
            .filter(|(_, y)| *y == c_y)
            .filter(|(x, y)| map.get(&(*x, *y)).is_none());

            let set = cant_be_here.map(|x| *x).collect::<HashSet<(i128,i128)>>();

            //println!("{:?}", closest_map.keys());

            for x in 0..=max_v {
                if set.get(&(x,c_y)).is_none() && closest_map.get(&(x,c_y)).is_none() {
                    g_x = x;
                    g_y = c_y;
                    println!("possible here: {},{} ({})",g_x,g_y,c_y);
                }
        }

    }
    static MULT: i128 = 4000000;
    println!("Viable {},{}", g_x, g_y);
    (g_x * MULT) + g_y
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"
    }

    #[test]
    fn day15_part1() {
        assert_eq!(part1_param(&day15_parse(get_test_input()), 10), 26);
    }

    #[test]
    fn day15_part2() {
        assert_eq!(part2_param(&day15_parse(get_test_input()), 20), 56000011);
    }
}
