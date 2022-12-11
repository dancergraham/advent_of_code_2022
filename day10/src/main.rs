#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = "addx 15
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
noop";

    #[test]
    fn test_part_1() {
        use super::*;
        let answer = part_1(TEST_INPUT);
        println!("The test answer to Part 1 is {}", answer);
        assert_eq!(answer, 13140);
    }

    #[test]
    fn test_part_2() {
        use super::*;
        let answer = part_2(TEST_INPUT);
        println!("The test answer to Part 2 is {}", answer);
        assert_eq!(answer, 1);
    }
}
fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    let path = Path::new("input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
            println!("The answer to Part 1 is {}", part_1(&s));
            println!("The answer to Part 2 is {}", part_2(&s));
        }
    }
}

fn part_1(input: &str) -> i32 {
    let mut cycle = 0;
    let mut register_x = 1;
    let mut signal_strength_sum = 0;
    let mut scores = 0; // spot multiples of twenty

    for line in input.lines() {
        let mut increment = 0;
        let mut duration = 0;
        let values: Vec<&str> = line.split(' ').collect();
        let operation = values[0];
        if operation == "noop" {
            duration = 1;
        } else if operation == "addx" {
            duration = 2;
            increment = values[1]
                .parse::<i32>()
                .expect("addx is always followed by an increment");
        }
        cycle = cycle + duration;
        if cycle / 20 > scores {
            scores = scores + 1;
            signal_strength_sum = signal_strength_sum + (scores * 20) * register_x;
            scores = scores + 1;
        }
        register_x = register_x + increment;
    }
    return signal_strength_sum;
}

fn part_2(input: &str) -> isize {
    let mut cycle: i32 = 0;
    let mut remaining_duration = 0;
    let mut register_x = 1;
    let mut increment = 0;
    let mut crt_output = String::from("");
    let mut instructions = input.lines().peekable();
    while instructions.peek().is_some() {
        cycle = cycle + 1;
        if remaining_duration == 0 {
            register_x = register_x + increment;
            increment = 0;
            let values: Vec<&str> = instructions
                .next()
                .expect("there is one")
                .split(" ")
                .collect();
            let operation = values[0];
            if operation == "noop" {
                remaining_duration = 1;
            } else if operation == "addx" {
                remaining_duration = 2;
                increment = values[1].parse::<i32>().unwrap();
            }
        }
        if ((cycle - 1) % 40).abs_diff(register_x) <= 1 {
            crt_output.push('#')
        } else {
            crt_output.push(' ')
        }
        remaining_duration = remaining_duration - 1;
    }
    for i in 0..6 {
        println!("{}", &crt_output[40 * i..40 * (i + 1)]);
    }
    return 1;
}
