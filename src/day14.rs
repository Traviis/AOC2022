
type InputType = Vec<&str>;
type OutputType = u64;

#[aoc_generator(day14)]
fn day14_parse(input: &str) -> InputType {
    todo!();
}

#[aoc(day14, part1)]
pub fn part1(input: &InputType) -> OutputType {
    todo!();
}

#[aoc(day14, part2)]
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
    fn day14_part1() {
        assert_eq!(part1(&day14_parse(get_test_input())), 0);
    }

    #[test]
    fn day14_part2() {
        assert_eq!(part2(&day14_parse(get_test_input())), 0);
    }
}
