
type InputType = Vec<&str>;
type OutputType = u64;

#[aoc_generator(day10)]
fn day10_parse(input: &str) -> InputType {
    todo!();
}

#[aoc(day10, part1)]
pub fn part1(input: &InputType) -> OutputType {
    todo!();
}

#[aoc(day10, part2)]
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
    fn day10_part1() {
        assert_eq!(part1(&day10_parse(get_test_input())), 0);
    }

    #[test]
    fn day10_part2() {
        assert_eq!(part2(&day10_parse(get_test_input())), 0);
    }
}
