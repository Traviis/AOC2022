use std::collections::HashSet;

type InputType = Vec<char>;
type OutputType = usize;

#[aoc_generator(day6)]
fn day6_parse(input: &str) -> InputType {
    input.chars().collect()
}

fn gen_solve(input: &InputType, win_size: usize) -> OutputType {
    let seq = input
        .windows(win_size)
        .enumerate()
        .find(|(_, slice)| {
            (*slice).iter().collect::<HashSet<_>>().iter().count() == win_size
            //They're all unique if the hashset is unique...
            // Original attempt was just running all the comparisions on the 4, but 14 is too
            // large for that, and while this is a little slow, it's not that slow
        })
        .unwrap();

    seq.0 + win_size
}

#[aoc(day6, part1)]
pub fn part1(input: &InputType) -> OutputType {
    gen_solve(input, 4)
}

#[aoc(day6, part2)]
pub fn part2(input: &InputType) -> OutputType {
    gen_solve(input, 14)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day6_part1() {
        assert_eq!(part1(&day6_parse("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 7);
        assert_eq!(part1(&day6_parse("nppdvjthqldpwncqszvftbrmjlhg")), 6);
        assert_eq!(part1(&day6_parse("bvwbjplbgvbhsrlpgdmjqwftvncz")), 5);
        assert_eq!(part1(&day6_parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 10);
        assert_eq!(part1(&day6_parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 11);
    }

    #[test]
    fn day6_part2() {
        assert_eq!(part2(&day6_parse("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 19);
        assert_eq!(part2(&day6_parse("bvwbjplbgvbhsrlpgdmjqwftvncz")), 23);
        assert_eq!(part2(&day6_parse("nppdvjthqldpwncqszvftbrmjlhg")), 23);
        assert_eq!(part2(&day6_parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 29);
        assert_eq!(part2(&day6_parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 26);
    }
}
