use std::collections::HashSet;
use std::iter;

type InputType = Vec<Command>;
type OutputType = usize;

pub enum Command {
    L(i8),
    R(i8),
    D(i8),
    U(i8),
}

impl Command {
    fn new(s: &str) -> Self {
        let mut it = s.split(" ");
        let ch = it.next().unwrap();
        let val = it.next().unwrap().parse::<i8>().unwrap();
        match ch {
            "D" => Command::D(val),
            "U" => Command::U(val),
            "R" => Command::R(val),
            "L" => Command::L(val),
            _ => panic!("Not my guy"),
        }
    }

    fn val(&self) -> i8 {
        *match self {
            Command::L(n) => n,
            Command::U(n) => n,
            Command::D(n) => n,
            Command::R(n) => n,
        }
    }

    fn get_diff_iter(&self) -> impl Iterator<Item = (i32, i32)> {
        match self {
            //Lame, but I have to homogenize the types by collecting them
            //Probably a better way to do this, but this works for now
            Command::L(n) => iter::repeat(-1)
                .take(*n as usize)
                .zip(iter::repeat(0))
                .collect::<Vec<_>>()
                .into_iter(),
            Command::R(n) => iter::repeat(1)
                .take(*n as usize)
                .zip(iter::repeat(0))
                .collect::<Vec<_>>()
                .into_iter(),
            Command::D(n) => iter::repeat(0)
                .zip(iter::repeat(-1).take(*n as usize))
                .collect::<Vec<_>>()
                .into_iter(),
            Command::U(n) => iter::repeat(0)
                .zip(iter::repeat(1).take(*n as usize))
                .collect::<Vec<_>>()
                .into_iter(),
        }
    }
}

fn update_pos(head: &(i32, i32), tail: &mut (i32, i32)) {
    if !close_enough(head, tail) {
        //Determine which direction
        // Brute force it for now
        // cardinal directions
        if head.0 == tail.0 {
            // same x
            if head.1 > tail.1 {
                tail.1 += 1;
            } else {
                tail.1 -= 1;
            }
        } else if head.1 == tail.1 {
            if head.0 > tail.0 {
                tail.0 += 1;
            } else {
                tail.0 -= 1;
            }
        } else if head.0 > tail.0 && head.1 > tail.1 {
            //Up and right
            tail.0 += 1;
            tail.1 += 1;
        } else if head.0 > tail.0 && head.1 < tail.1 {
            //Down and right
            tail.0 += 1;
            tail.1 -= 1;
        } else if head.0 < tail.0 && head.1 < tail.1 {
            //Down and left
            tail.0 -= 1;
            tail.1 -= 1;
        } else {
            //up and left
            tail.0 -= 1;
            tail.1 += 1;
        }

        //save tail
    }
}

/*
fn distance(head: &(i32,i32), tail: &(i32,i32)) -> usize {
    //seems expensive to calculate this... but whatever, optimize later
    sqrtf64(powf64(tail.0 - head.0,2) + powf64(tail.1 - head.1,2)).floor()
}
*/
fn close_enough(head: &(i32, i32), tail: &(i32, i32)) -> bool {
    // Are the points within 1 of eachother?
    //(head.0 - tail.0).abs() > 1 || (head.0 - tail.0).abs() > 1
    if head.0 == tail.0 {
        (head.1 - tail.1).abs() <= 1
    } else if head.1 == tail.1 {
        (head.0 - tail.0).abs() <= 1
    } else if (head.1 - tail.1).abs() == 1 && (head.0 - tail.0).abs() == 1 {
        true
    } else {
        false
    }
}

#[aoc_generator(day9)]
fn day9_parse(input: &str) -> InputType {
    input
        .lines()
        .map(|line| Command::new(line))
        .collect::<Vec<_>>()
}

#[aoc(day9, part1)]
pub fn part1(input: &InputType) -> OutputType {
    //Unique positions
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    tail_positions.insert((0, 0)); //Initial is a position

    //initial positions
    let mut tail: (i32, i32) = (0, 0);
    let mut head: (i32, i32) = (0, 0);

    for cmd in input.iter() {
        //update head position
        //
        let diffs = cmd.get_diff_iter();

        diffs.for_each(|(cx, cy)| {
            //Update head
            head.0 += cx as i32;
            head.1 += cy as i32;

            update_pos(&head, &mut tail);

            //Update tail
            //The tail moves iif it's not touching, it always moves in one of 8 directions
            tail_positions.insert(tail);
        });
    }

    tail_positions.iter().count()
}

#[aoc(day9, part2)]
pub fn part2(input: &InputType) -> OutputType {
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    tail_positions.insert((0, 0)); //Initial is a position
                                   //

    static TAIL_NUM: usize = 9;
    //initial positions
    let mut head: (i32, i32) = (0, 0);
    let mut tails = iter::repeat((0, 0)).take(TAIL_NUM).collect::<Vec<_>>();

    for cmd in input.iter() {
        let diffs = cmd.get_diff_iter();

        diffs.for_each(|(cx, cy)| {
            head.0 += cx as i32;
            head.1 += cy as i32;
            //initial movement
            update_pos(&head, &mut tails.iter_mut().next().unwrap());

            //I would love to do this, but you can't get mutable due to how iterator is implemented
            //here, oh well, just do it the dumb way
            //tails.windows(2).for_each(|pair| {
            //    match pair {
            //        &[cur,mut next] => update_pos(&cur, &mut next),
            //        _ => panic!("huh")
            //    }
            //});
            for idx in 0..TAIL_NUM - 1 {
                let cur = tails.get(idx).unwrap().clone();
                let next = tails.get_mut(idx + 1).unwrap();
                update_pos(&cur, next);
                //We know when it's going to stop, so... just cheat
            }
            //println!("{:?}", tails);

            tail_positions.insert(*tails.last().unwrap());
        });
    }
    tail_positions.iter().count()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
    }
    fn get_test_input_2() -> &'static str {
        "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
    }

    #[test]
    fn day9_part1() {
        assert_eq!(part1(&day9_parse(get_test_input())), 13);
    }

    #[test]
    fn day9_part2() {
        assert_eq!(part2(&day9_parse(get_test_input_2())), 36);
    }
}
