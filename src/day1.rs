use std::fs;

// Let's over-engineer this for explicitness.
type Calories = u64;
// Ended up not needing this, since both parts just asked for the sums in the backpacks.
type ElfBackpack = Vec<Calories>;

#[aoc_generator(day1)]
fn day1_parse(input: &str) -> Vec<ElfBackpack> {
    input
        .split("\n\n")
        .map(|bp| {
            bp.split("\n")
                .map(|n| n.parse::<Calories>().unwrap())
                .collect::<ElfBackpack>()
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<ElfBackpack>) -> u64 {
    input
        .iter()
        .map(|bp| bp.iter().sum::<Calories>())
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<ElfBackpack>) -> u64 {
    let mut sums: Vec<u64> = input.iter().map(|bp| bp.iter().sum::<Calories>()).collect();
    sums.sort();
    sums.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
    }

    #[test]
    fn day1_part1() {
        assert_eq!(part1(&day1_parse(get_test_input())), 24000);
    }

    #[test]
    fn day1_part2() {
        assert_eq!(part2(&day1_parse(get_test_input())), 45000);
    }
}
