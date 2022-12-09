
type InputType = Vec<&str>;
type OutputType = u64;

#[aoc_generator(day9)]
fn day9_parse(input: &str) -> InputType {
    todo!();
}

#[aoc(day9, part1)]
pub fn part1(input: &InputType) -> OutputType {
    todo!();
}

#[aoc(day9, part2)]
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
    fn day9_part1() {
        assert_eq!(part1(&day9_parse(get_test_input())), 0);
    }

    #[test]
    fn day9_part2() {
        assert_eq!(part2(&day9_parse(get_test_input())), 0);
    }
}
