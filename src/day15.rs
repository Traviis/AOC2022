extern crate lazy_static;
use self::lazy_static::lazy_static;
extern crate regex;
use self::regex::Regex;
use std::collections::{HashMap, HashSet};
extern crate rayon;
use self::rayon::prelude::*;
use std::sync::Mutex;
use std::sync::Arc;

pub enum Item {
    Sensor,
    Beacon,
    Nothing,
}

type InputType = (HashMap<(i64, i64), Item>, HashMap<(i64, i64), (i64, i64)>);
type OutputType = usize;

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
            .map(|c| c.unwrap().as_str().parse::<i64>().unwrap());

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
    map: &HashMap<(i64, i64), Item>,
    max_x: usize,
    max_y: usize,
    sens_range: &HashSet<(i64, i64)>,
) {
    for y in -2..=max_y as i64 {
        for x in -2..=max_x as i64 {
            print!(
                "{}",
                match map.get(&(x as i64, y as i64)).unwrap_or(&Item::Nothing) {
                    Item::Sensor => "S",
                    Item::Beacon => "B",
                    Item::Nothing =>
                        if sens_range.get(&(x as i64, y as i64)).is_some() {
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

fn manhatten((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn naive_determine_sensor_range(
    (s_x, s_y): (i64, i64),
    (b_x, b_y): (i64, i64),
) -> HashSet<(i64, i64)> {
    //There is probably a clever way of calculating this, but brute force this bad boy!
    let dist = manhatten((s_x, s_y), (b_x, b_y));

    let mut out = HashSet::new();
    for m_y in s_y - dist..=s_y + dist {
        for m_x in s_x - dist..=s_x + dist {
            if manhatten((m_x, m_y), (s_x, s_y)) <= dist {
                out.insert((m_x, m_y));
            }
        }
    }
    out
}
fn determine_sensor_range((s_x, s_y): (i64, i64), (b_x, b_y): (i64, i64)) -> HashSet<(i64, i64)> {
    let dist = manhatten((s_x, s_y), (b_x, b_y));
    todo!();
}

#[aoc(day15, part1)]
pub fn part1((map, closest_map): &InputType) -> OutputType {
    //let (b_x, b_y) = closest_map.get(&(8, 7)).unwrap();
    //print_debug_map(map, 25, 22, &determine_sensor_range((8, 7), (*b_x, *b_y)));
    // let all_ranges : HashSet<(i64,i64)>= closest_map
    //     .par_iter()
    //     .map(|((s_x, s_y), (b_x, b_y))| determine_sensor_range((*s_x, *s_y), (*b_x, *b_y)))
    //     .reduce(|| HashSet::new(), |a: HashSet<(i64,i64)>, b: HashSet<(i64,i64)>| {
    //         a.union(&b).cloned().collect()
    //     }
    //     );
    //fold(HashSet::new(), |acc, x| acc.union(&x).cloned().collect());
    let mut all_ranges = Arc::new(Mutex::new(HashSet::new()));

    closest_map
        .par_iter()
        .map(|((s_x, s_y), (b_x, b_y))| naive_determine_sensor_range((*s_x, *s_y), (*b_x, *b_y)))
        .for_each(|h_set| {
            let ranges = Arc::clone(&all_ranges);
            let mut new_set = ranges.lock().unwrap();
            *new_set = (*new_set).union(&h_set).map(|x| *x).collect::<HashSet<(i64,i64)>>();
        });

    println!("Ranges");

    let homies = all_ranges
        .lock().unwrap();

    homies
        .iter()
        .filter(|(_, y)| *y == 10)
        .filter(|(x, y)| map.get(&(*x, *y)).is_none())
        .count()
}

#[aoc(day15, part2)]
pub fn part2(input: &InputType) -> OutputType {
    todo!();
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
        assert_eq!(part1(&day15_parse(get_test_input())), 26);
    }

    #[test]
    fn day15_part2() {
        assert_eq!(part2(&day15_parse(get_test_input())), 0);
    }
}
