extern crate lazy_static;
use self::lazy_static::lazy_static;
extern crate regex;
use self::regex::Regex;
use std::collections::{HashMap, HashSet};
extern crate rayon;
use self::rayon::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;

use std::convert::TryFrom;


#[derive(Debug)]
pub enum Item {
    Sensor,
    Beacon,
    Nothing,
}

type InputType = (HashMap<(i32, i32), Item>, HashMap<(i32, i32), (i32, i32)>);
type OutputType = i32;

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
            .map(|c| c.unwrap().as_str().parse::<i32>().unwrap());

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
    map: &HashMap<(i32, i32), Item>,
    max_x: usize,
    max_y: usize,
    sens_range: &HashSet<(i32, i32)>,
) {
    for y in -2..=max_y as i32 {
        for x in -2..=max_x as i32 {
            print!(
                "{}",
                match map.get(&(x as i32, y as i32)).unwrap_or(&Item::Nothing) {
                    Item::Sensor => "S",
                    Item::Beacon => "B",
                    Item::Nothing =>
                        if sens_range.get(&(x as i32, y as i32)).is_some() {
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

fn manhatten((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn bounded_naive_determine_sensor_range(
    (s_x, s_y): (i32, i32),
    (b_x, b_y): (i32, i32),
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
) -> HashSet<(i32, i32)> {
    let dist = manhatten((s_x, s_y), (b_x, b_y));

    //We only care about a very specific y coordinate

    //TODO: This can be optimized more, but let's just try to brute force it with checking the row
    let mut out = HashSet::new();
    out.insert((s_x, s_y));
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

fn find_sensor_range_intersections(
    closest_map: &HashMap<(i32, i32), (i32, i32)>,
    max_coord: i32,
) -> i32 {
    let sensors = &closest_map
        .iter()
        .map(|((sx, sy), (bx, by))| ((i32::try_from(*sx).unwrap(), i32::try_from(*sy).unwrap()), (i32::try_from(*bx).unwrap(), i32::try_from(*by).unwrap())))
        .collect::<Vec<((i32, i32), (i32, i32))>>()[..];

    //This is failing..? Is my input bad?

    //Wish I could take credit for this, but I was just tired of this problem and found someone
    //elses solution: https://github.com/emlun/adventofcode-2022/blob/9dadc35bb4ebbb8352f9525f0eeaf2ea2babd766/src/days/day15.rs#L57-L91
    //Reddit thread with explanation: https://www.reddit.com/r/adventofcode/comments/zmw9d8/2022_day_15_part_2_speed_up_your_solution_60000x/
    for (i1, ((sx1, sy1), (bx1, by1))) in sensors.iter().enumerate() {
        let r1: i32 = i32::try_from(sx1.abs_diff(*bx1) + sy1.abs_diff(*by1)).unwrap();

        for ((sx2, sy2), (bx2, by2)) in sensors[i1 + 1..].iter() {
            let r2: i32 = i32::try_from(sx2.abs_diff(*bx2) + sy2.abs_diff(*by2)).unwrap();
            let b1s: [i32; 4] = [
                r2 + sx2 + sy2 + 1,
                -r2 + sx2 + sy2 - 1,
                r2 + sx2 + sy2 + 1,
                -r2 + sx2 + sy2 - 1,
            ];
            let b2s: [i32; 4] = [
                -r1 + sx1 - sy1 - 1,
                -r1 + sx1 - sy1 - 1,
                r1 + sx1 - sy1 + 1,
                r1 + sx1 - sy1 + 1,
            ];

            for (b1, b2) in b1s.iter().zip(b2s) {
                let x = (b1 + b2) / 2;
                let y = (b1 - b2) / 2;

                if (0..=max_coord).contains(&x)
                    && (0..=max_coord).contains(&y)
                    && sensors.iter().all(|((sx, sy), (bx, by))| {
                        let r = sx.abs_diff(*bx) + sy.abs_diff(*by);
                        let d = sx.abs_diff(x) + sy.abs_diff(y);
                        d > r
                    })
                {
                    return i32::from(x) * 4000000 + i32::from(y);
                }
            }
        }
    }

    // If solution is not on the boundary of two sensors,
    // it must be in a corner of the permitted region.
    for x in [0, max_coord] {
        for y in [0, max_coord] {
            if sensors.iter().all(|((sx, sy), (bx, by))| {
                let r = sx.abs_diff(*bx) + sy.abs_diff(*by);
                let d = sx.abs_diff(x) + sy.abs_diff(y);
                d > r
            }) {
                return i32::from(x) * 4000000 + i32::from(y);
            }
        }
    }

    unimplemented!();
}

fn naive_determine_sensor_range(
    (s_x, s_y): (i32, i32),
    (b_x, b_y): (i32, i32),
    cared_y: i32,
) -> HashSet<(i32, i32)> {
    //There is probably a clever way of calculating this, but brute force this bad boy!
    bounded_naive_determine_sensor_range(
        (s_x, s_y),
        (b_x, b_y),
        i32::MIN,
        i32::MAX,
        cared_y,
        cared_y,
    )
}
fn determine_sensor_range((s_x, s_y): (i32, i32), (b_x, b_y): (i32, i32)) -> HashSet<(i32, i32)> {
    let dist = manhatten((s_x, s_y), (b_x, b_y));
    todo!();
}

#[aoc(day15, part1)]
pub fn part1(inp: &InputType) -> OutputType {
    part1_param(inp, 2000000)
}


pub fn part1_param((map, closest_map): &InputType, cared_y: i32) -> OutputType {
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
                .collect::<HashSet<(i32, i32)>>();
        });

    let cant_be = all_ranges.lock().unwrap();

    cant_be
        .iter()
        .filter(|(_, y)| *y == cared_y)
        .filter(|(x, y)| map.get(&(*x, *y)).is_none())
        .count() as i32
}

#[aoc(day15, part2)]
pub fn part2(inp: &InputType) -> OutputType {
    part2_param(inp, 2000000)
}
pub fn part2_param((map, closest_map): &InputType, max_v: i32) -> OutputType {
    find_sensor_range_intersections(closest_map, max_v)
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
