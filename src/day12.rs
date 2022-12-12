
type InputType = Vec<&str>;
type OutputType = u64;

#[aoc_generator(day12)]
fn day12_parse(input: &str) -> InputType {
    todo!();
}

#[aoc(day12, part1)]
pub fn part1(input: &InputType) -> OutputType {
    todo!();
}

#[aoc(day12, part2)]
pub fn part2(input: &InputType) -> OutputType {
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        todo!();
    }

    #[test]
    fn day12_part1() {
        assert_eq!(part1(&day12_parse(get_test_input())), 0);
    }

    #[test]
    fn day12_part2() {
        assert_eq!(part2(&day12_parse(get_test_input())), 0);
    }
}
