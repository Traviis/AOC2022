use std::collections::{HashMap, HashSet, VecDeque};

type InputType = Vec<Vec<char>>;
type OutputType = usize;

type Coordinate = (usize, usize);

#[aoc_generator(day12)]
fn day12_parse(input: &str) -> InputType {
    input
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>()
}

fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    find_char('S', grid)
}

fn find_end(grid: &Vec<Vec<char>>) -> (usize, usize) {
    find_char('E', grid)
}

fn find_char(ch: char, grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, ve) in grid.iter().enumerate() {
        for (x, c) in ve.iter().enumerate() {
            if ch == *c {
                return (x, y);
            }
        }
    }
    panic!("S not found")
}

fn find_candidates(
    input: &InputType,
    (x, y): Coordinate,
    max_x: usize,
    max_y: usize,
) -> Vec<Coordinate> {
    //Don't care about diagonals, just left, right, up, down
    let mut candidates = vec![];
    let cur_height = input[y][x] as i32; //cast char to some value
                                         //println!("Max x: {} max y: {}", max_x, max_y);
    for (c_x, c_y) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let cand = (x as i32 + c_x, y as i32 + c_y);
        if cand.0 > max_x as i32 || cand.1 > max_y as i32 || cand.0 < 0 || cand.1 < 0 {
            continue;
        }
        // colummn major
        let val = *(input
            .get(cand.1 as usize)
            .unwrap()
            .get(cand.0 as usize)
            .unwrap()) as i32;
        //You can go down any number of levels, but can only go up 1
        if cur_height - val as i32 >= -1 {
            candidates.push(cand);
        }
    }

    candidates
        .iter()
        .map(|(x, y)| (*x as usize, *y as usize))
        .collect()
}

pub fn dijs(
    input: &mut InputType,
    (s_x, s_y): (usize, usize),
    (e_x, e_y): (usize, usize),
    max_x: usize,
    max_y: usize,
) -> OutputType {
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut to_examine: VecDeque<Coordinate> = VecDeque::new();
    let mut distance: HashMap<Coordinate, usize> = HashMap::new();
    let mut previous: HashMap<Coordinate, Coordinate> = HashMap::new();

    distance.insert((s_x, s_y), 0);

    static ARBITRARY_HIGH_COST: usize = 5000;

    to_examine.push_back((s_x, s_y));

    while let Some(u_node) = to_examine.pop_front() {
        //Technically I need the minimum distance for the available vertices
        visited.insert((u_node.0, u_node.1));
        let candidate_nodes = find_candidates(&input, u_node, max_x, max_y);
        //       println!("Candidates for {:?}: {:?}", u_node, candidate_nodes);
        for v_node in candidate_nodes.iter() {
            if visited.get(v_node).is_some() {
                continue;
            }
            let temp_distance = (*distance.get(&u_node).unwrap_or(&ARBITRARY_HIGH_COST)) + 1;
            if temp_distance < *distance.get(v_node).unwrap_or(&ARBITRARY_HIGH_COST) {
                distance.insert(*v_node, temp_distance);
                previous.insert(*v_node, u_node);
                to_examine.push_back(*v_node);
            }
        }
    }
    //    println!("dist: {:?}", distance);
    let dist = *(distance.get(&(e_x, e_y)).unwrap_or(&ARBITRARY_HIGH_COST));
    // println!("Start ({},{}) => {}", s_x, s_y, dist);
    dist
}

#[aoc(day12, part1)]
pub fn part1(input: &InputType) -> OutputType {
    let mut input = input.clone();
    let max_x = input.get(0).unwrap().iter().count() - 1;
    let max_y = input.iter().count() - 1;
    let (e_x, e_y) = find_end(&input);
    let (s_x, s_y) = find_start(&input);

    input[s_y][s_x] = 'a';
    input[e_y][e_x] = 'z'; //set proper heights
                           //
    dijs(&mut input, (s_x, s_y), (e_x, e_y), max_x, max_y)
}

#[aoc(day12, part2)]
pub fn part2(input: &InputType) -> OutputType {
    let mut input = input.clone();
    let max_x = input.get(0).unwrap().iter().count() - 1;
    let max_y = input.iter().count() - 1;
    let (e_x, e_y) = find_end(&input);
    let (s_x, s_y) = find_start(&input); //This is the old start, we actually don't care
    input[s_y][s_x] = 'a';
    input[e_y][e_x] = 'z'; //set proper heights
    let mut candidates = Vec::new();
    for x in 0..=max_x {
        for y in 0..=max_y {
            if input[y][x] == 'a' {
                candidates.push((x, y));
            }
        }
    }

    candidates
        .iter()
        .map(|(x, y)| dijs(&mut input, (*x, *y), (e_x, e_y), max_x, max_y))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
    }

    #[test]
    fn day12_part1() {
        assert_eq!(part1(&day12_parse(get_test_input())), 31);
    }

    #[test]
    fn day12_part2() {
        assert_eq!(part2(&day12_parse(get_test_input())), 29);
    }
}
