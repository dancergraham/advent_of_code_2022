#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        let test_input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        use super::*;
        let answer = part_1(test_input);
        println!("The test answer to Part 1 is {}", answer);
        assert_eq!(answer, "CMZ");
        let answer = part_2(test_input);
        println!("The test answer to Part 2 is {}", answer);
        assert_eq!(answer, "MCD");
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

fn part_1(input: &str) -> String {
    let mut sections = input.split("\n\n");
    let stackstrings = sections.next().expect("");
    let n_stacks = stackstrings
        .split("   ")
        .last()
        .expect("Final number is the number of stacks")
        .trim()
        .parse::<usize>()
        .unwrap();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; n_stacks];
    for line in stackstrings.split("\n") {
        let mut chars = line.chars();
        chars.next();
        let char = chars.next().expect("at least one stack");
        if char != ' ' {
            stacks[0].push(char);
        }
        for i in 1..n_stacks {
            let char = chars.nth(3).unwrap();
            if char != ' ' {
                stacks[i].push(char);
            }
        }
    }
    let instructions = sections.next().expect("");
    for line in instructions.lines() {
        let items: Vec<&str> = line.split(' ').collect();
        let n_items = items[1].parse::<usize>().unwrap();
        let stack_from = items[3].parse::<usize>().unwrap() - 1;
        let stack_to = items[5].parse::<usize>().unwrap() - 1;
        for _i in 0..n_items {
            let item = stacks[stack_from].pop().expect("it will be fine");
            stacks[stack_to].push(item);
        }
    }
    let mut answer_vec = vec![];
    for i in 0..n_stacks {
        answer_vec.push(stacks[i].pop().expect("at least one item on each stack"))
    }
    let answer: String = answer_vec.iter().collect();
    return answer;
}

fn part_2(input: &str) -> String {
    let mut sections = input.split("\n\n");
    let stackstrings = sections.next().expect("");
    let n_stacks = stackstrings
        .split("   ")
        .last()
        .expect("Final number is the number of stacks")
        .trim()
        .parse::<usize>()
        .unwrap();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; n_stacks];
    for line in stackstrings.lines() {
        let mut chars = line.chars();
        chars.next();
        let char = chars.next().expect("at least one stack");
        if char != ' ' {
            stacks[0].push(char);
        }
        for i in 1..n_stacks {
            let char = chars.nth(3).unwrap();
            if char != ' ' {
                stacks[i].push(char);
            }
        }
    }
    let instructions = sections.next().expect("");
    for line in instructions.lines() {
        let items: Vec<&str> = line.split(' ').collect();
        let n_items = items[1].parse::<usize>().unwrap();
        let stack_from = items[3].parse::<usize>().unwrap() - 1;
        let stack_to = items[5].parse::<usize>().unwrap() - 1;
        let split_point = stacks[stack_from].len() - n_items;
        let mut split_items = stacks[stack_from].split_off(split_point);
        stacks[stack_to].append(split_items.as_mut());
    }
    let mut answer_vec = vec![];
    for i in 0..n_stacks {
        answer_vec.push(stacks[i].pop().expect("at least one item on each stack"))
    }
    let answer: String = answer_vec.iter().collect();
    return answer;
}
