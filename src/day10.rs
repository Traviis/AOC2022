use std::collections::HashSet;

#[derive(Debug)]
pub enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn new(line: &str) -> Self {
        let mut words = line.split(" ");
        match words.next().unwrap() {
            "noop" => Instruction::Noop,
            "addx" => Instruction::AddX(words.next().unwrap().parse::<i32>().unwrap()),
            _ => panic!("Illegal Instruction"),
        }
    }
}

type InputType = Vec<Instruction>;
type OutputType = i64;

#[aoc_generator(day10)]
fn day10_parse(input: &str) -> InputType {
    input
        .split("\n")
        .map(|line| Instruction::new(line))
        .collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &InputType) -> OutputType {
    simulate(input, false).0
}

pub fn simulate(input: &InputType, part2: bool) -> (OutputType, String) {
    let mut screen_output: HashSet<i64> = HashSet::new(); //Just the lit pixels

    let mut cycle: i64 = 1;
    let mut signal_strength: i64 = 0;

    let mut x_val: i64 = 1;

    let mut current_inst: Option<&Instruction> = None;
    let mut cycles_on_inst = 0;

    static SAMPLE_INC: i64 = 40;
    let mut next_sample_time: i64 = 20;

    let mut inst_iter = input.iter().peekable();

    let mut last_cycle = false;

    loop {
        //        println!("X {} cycle {}",x_val, cycle);

        if inst_iter.peek().is_none() {
            last_cycle = true;
        }

        if cycle == next_sample_time {
            signal_strength += cycle as i64 * x_val;
            //println!("Sampling on {} new val {}", cycle, cycle as i64 * x_val);
            next_sample_time += SAMPLE_INC;
        }

        if current_inst.is_none() && !last_cycle {
            current_inst = Some(inst_iter.next().unwrap());
            cycles_on_inst = 0;
            //println!("Start Exec on cycle {} of {:?}", cycle, current_inst);
        }

        //draw
        if part2 {
            let current_line = (cycle - 1) / SAMPLE_INC;
            //range == current_line * 40 -> (current_line * 40) + 40
            let center = (current_line * SAMPLE_INC) + x_val;

            let current_pixel_candidate = (current_line * SAMPLE_INC) + (cycle - 1) % SAMPLE_INC;

            if current_pixel_candidate >= center - 1 && current_pixel_candidate <= center + 1 {
                screen_output.insert(current_pixel_candidate);
            }
        }

        cycle += 1;
        if current_inst.is_some() {
            cycles_on_inst += 1;
        }

        //Impl logic
        if cycles_on_inst == 2 && current_inst.is_some() {
            if let Instruction::AddX(n) = current_inst.unwrap() {
                x_val += *n as i64;
                current_inst = None;
                //println!("Add {} new = {}",n,x_val);
            }
        }

        if cycles_on_inst == 1 && current_inst.is_some() {
            if let Instruction::Noop = current_inst.unwrap() {
                //do nothing
                current_inst = None;
            }
        }

        if last_cycle {
            break;
        }
    }

    let mut string_output = vec![];
    if part2 {
        //print screen
        for x in 0..cycle - 2 {
            //honestly, too lazy to figure out where that off by 2 is from

            //Uncomment these if you need to see the output
            if screen_output.get(&x).is_some() {
                //print!("#");
                string_output.push('#');
            } else {
                //print!(".");
                string_output.push('.');
            }
            if (x + 1) % 40 == 0 {
                //println!("");
            }
        }
    }

    if !part2 {
        (signal_strength, "".to_string())
    } else {
        //Since I'm not going to write OCR, let's just detect the number of lit pixels per line,
        //multiplied by each other, save that as the correct test result (I could output differnt
        //things based on the G
        (0, string_output.iter().collect::<String>())
    }
}

#[aoc(day10, part2)]
pub fn part2(input: &InputType) -> String {
    simulate(input, true).1
}

#[cfg(test)]
mod tests {

    use super::*;


    fn get_test_input() -> &'static str {
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
    }

    #[test]
    fn day10_part1() {
        assert_eq!(part1(&day10_parse(get_test_input())), 13140);
    }

    #[test]
    fn day10_part2() {
        let correct_output = "##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######.....";
        assert_eq!(part2(&day10_parse(get_test_input())), correct_output);
    }
}
