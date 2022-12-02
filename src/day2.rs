type StrategyGuide = Vec<(Hand, Hand)>;

type InputType = StrategyGuide;
type OutputType = u64;

//Thoughts:
// 1. I didn't end up needing the stored string in the variant
// 2. Implementing this all in enums gets a little verbose, I suppose I might have been able to do
//    it with some variant of PartialOrd to allow comparisons of the values, but it gets a little
//    strange given that it's a modular comparison (so I would have to ensure that the function
//    never got used for anything other than a direct comparison, and not like sorting. I could
//    have just really, instead of enums, used the point values of the hand to determine their
//    identity, I have a sneaking suspicion there is a clever math trick if I examined the
//    numbers close enough.
// 3. I would have liked to have done &str for the Hand, but then I have to introduce lifetimes,
//    and the aoc-runner didn't seem to really like that, or at least, I was having a hard time
//    parsing the error, so just copy, even though it loses out on performance.

#[derive(Debug, Clone)]
pub enum Hand {
    //Include the variant (A or Y for each, might not be needed)
    Rock(String),
    Paper(String),
    Scissor(String),
}

impl PartialEq for Hand {
    //Would be cool if there was an easier way to do this, but this seems to be the best way to
    //throw out the additional information (the string variant)
    fn eq(&self, other: &Self) -> bool {
        match self {
            Hand::Rock(_) => {
                if let Hand::Rock(_) = other {
                    true
                } else {
                    false
                }
            }
            Hand::Paper(_) => {
                if let Hand::Paper(_) = other {
                    true
                } else {
                    false
                }
            }
            Hand::Scissor(_) => {
                if let Hand::Scissor(_) = other {
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

//Could maybe do this with some PartialOrd Stuff but lets get simple first
fn get_outcome(a: &Hand, b: &Hand) -> Outcome {
    if a == b {
        return Outcome::Draw;
    }
    match a {
        Hand::Rock(_) => {
            if let Hand::Scissor(_) = b {
                Outcome::Lose
            } else {
                Outcome::Win
            }
        }
        Hand::Paper(_) => {
            if let Hand::Rock(_) = b {
                Outcome::Lose
            } else {
                Outcome::Win
            }
        }
        Hand::Scissor(_) => {
            if let Hand::Paper(_) = b {
                Outcome::Lose
            } else {
                Outcome::Win
            }
        }
    }
}

fn round_outcome(a: &Hand, b: &Hand) -> u64 {
    let selected_shape_score: u64 = match b {
        Hand::Rock(_) => 1,
        Hand::Paper(_) => 2,
        Hand::Scissor(_) => 3,
    };

    let outcome: u64 = match get_outcome(a, b) {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    };

    //println!("{:?} {:?} => {:?} + {} = {}", a, b, outcome, selected_shape_score, outcome + selected_shape_score);

    outcome + selected_shape_score
}

//part 2
impl From<&String> for Outcome {
    fn from(c: &String) -> Self {
        match c.as_str() {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Unknown char {}", c),
        }
    }
}
fn convert_hand_to_outcome(b: &Hand) -> Outcome {
    //heh, since we mistranslated the column, just add a translation shim here You could of course
    //just not convert across like this and use Hand, but that's kind of gross.
    // it's really cool that I can use an or expression in a pattern and have it all resolve to the
    // same variable
    match b {
        Hand::Rock(a) | Hand::Scissor(a) | Hand::Paper(a) => a.into(),
    }
}
fn determine_play(a: &Hand, b: &Outcome) -> Hand {
    match b {
        Outcome::Draw => a.clone(), //given that the plays are different for the first column, this
                                    //technically has an incorrect string variant, but I never
                                    //check them so, who cares.
        Outcome::Win => match a {
            Hand::Rock(_) => Hand::Paper("Y".to_string()),
            Hand::Paper(_) => Hand::Scissor("Z".to_string()),
            Hand::Scissor(_) => Hand::Rock("X".to_string()),
        },
        Outcome::Lose => match a {
            Hand::Rock(_) => Hand::Scissor("Z".to_string()),
            Hand::Paper(_) => Hand::Rock("X".to_string()),
            Hand::Scissor(_) => Hand::Paper("Y".to_string()),
        },
    }
}

//Could use &str here, but the macro doesn't really expect to work well with lifetime specifiers,
//so be lazy for now
impl From<String> for Hand {
    fn from(c: String) -> Self {
        match c.as_str() {
            "A" | "X" => Hand::Rock(c),
            "B" | "Y" => Hand::Paper(c),
            "C" | "Z" => Hand::Scissor(c),
            _ => panic!("Unknown char {}", c),
        }
    }
}

#[aoc_generator(day2)]
fn day2_parse(input: &str) -> InputType {
    input
        .split("\n")
        .map(|line| {
            let mut it = line.split_whitespace().map(|c| c.to_owned().into());
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &InputType) -> OutputType {
    input.iter().map(|(r1, r2)| round_outcome(r1, r2)).sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &InputType) -> OutputType {
    input
        .iter()
        .map(|(r1, r2)| (r1, determine_play(&r1, &convert_hand_to_outcome(&r2))))
        .map(|(r1, r2)| round_outcome(&r1, &r2))
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "A Y
B X
C Z"
    }

    #[test]
    fn day2_part1() {
        assert_eq!(part1(&day2_parse(get_test_input())), 15);
    }

    #[test]
    fn day2_part2() {
        assert_eq!(part2(&day2_parse(get_test_input())), 12);
    }
}
