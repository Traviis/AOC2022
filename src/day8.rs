extern crate itertools;
use self::itertools::iproduct;

type InputType = Vec<Vec<u8>>;
type OutputType = i32;

#[aoc_generator(day8)]
fn day8_parse(input: &str) -> InputType {
    input
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<InputType>()
}

#[aoc(day8, part1)]
pub fn part1(input: &InputType) -> OutputType {
    let max_x = input[0].iter().count() as i32;
    let max_y = input.iter().count() as i32;
    //    println!("max_x {} max_y {}", max_x, max_y);
    iproduct!(1..max_x - 1, 1..max_y - 1).fold(0, |acc, (x, y)| {
        let cur_height = input[y as usize][x as usize];
        if (0..x).all(|cx| input[y as usize][cx as usize] < cur_height) {
            return acc + 1;
        }
        if (x + 1..max_x).all(|cx| input[y as usize][cx as usize] < cur_height) {
            return acc + 1;
        }
        if (0..y).all(|cy| input[cy as usize][x as usize] < cur_height) {
            return acc + 1;
        }
        if (y + 1..max_y).all(|cy| input[cy as usize][x as usize] < cur_height) {
            return acc + 1;
        }

        return acc;
    }) + (2 * max_x + 2 * max_y - 4)
}

#[aoc(day8, part2)]
pub fn part2(input: &InputType) -> OutputType {
    let max_x = input[0].iter().count() as i32;
    let max_y = input.iter().count() as i32;
    iproduct!(1..max_x - 1, 1..max_y - 1)
        .map(|(x, y)| {
            //println!("({},{})", x, y);
            let cur_height = input[y as usize][x as usize];

            //TODO: take_while with an enumerate? Could that work?
            let mut left = 0;
            for cx in (0..x).rev() {
                left += 1;
                if input[y as usize][cx as usize] >= cur_height {
                    break;
                }
            }
            let mut right = 0;
            for cx in x + 1..max_x {
                right += 1;
                if input[y as usize][cx as usize] >= cur_height {
                    break;
                }
            }

            let mut down = 0;
            for cy in (0..y).rev() {
                down += 1;
                if input[cy as usize][x as usize] >= cur_height {
                    break;
                }
            }

            let mut up = 0;
            for cy in y + 1..max_y {
                up += 1;
                if input[cy as usize][x as usize] >= cur_height {
                    break;
                }
            }

            left * right * down * up
        })
        .max()
        .unwrap() as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "30373
25512
65332
33549
35390"
    }

    #[test]
    fn day8_part1() {
        assert_eq!(part1(&day8_parse(get_test_input())), 21);
    }

    #[test]
    fn day8_part2() {
        assert_eq!(part2(&day8_parse(get_test_input())), 8);
    }
}
