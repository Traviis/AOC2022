
type InputType = Vec<&str>;
type OutputType = u64;

#[aoc_generator(day8)]
fn day8_parse(input: &str) -> InputType {
    todo!();
}

#[aoc(day8, part1)]
pub fn part1(input: &InputType) -> OutputType {
    todo!();
}

#[aoc(day8, part2)]
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
    fn day8_part1() {
        assert_eq!(part1(&day8_parse(get_test_input())), 0);
    }

    #[test]
    fn day8_part2() {
        assert_eq!(part2(&day8_parse(get_test_input())), 0);
    }
}
