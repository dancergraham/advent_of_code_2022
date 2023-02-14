fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    let test_input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
    let answer = part_1(test_input);
    println!("The test answer to Part 1 is {answer}");
    assert_eq!(answer, 157);

    let path = Path::new("input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
            let answer_part_1 = part_1(&s);
            println!("The answer to Part 1 is {answer_part_1}");
            let answer_part_2 = part_2(&s);
            println!("The answer to Part 2 is {answer_part_2}");
        }
    }
}

fn get_priority(item: char) -> i32 {
    if item.is_ascii_uppercase() {
        item as i32 - 38
    } else {
        item as i32 - 96
    }
}

fn part_1(input: &str) -> i32 {
    let mut answer = 0;
    let mut priority = 0;
    let lines = input.lines();
    for line in lines {
        let line_length = line.len();
        let first = &line[..line_length / 2];
        let second = &line[line_length / 2..];
        for item_1 in first.chars() {
            if second.contains(item_1) {
                priority = get_priority(item_1);
                break;
            }
        }
        answer += priority;
    }
    answer
}

fn part_2(input: &str) -> i32 {
    let mut answer = 0;
    let mut priority = 0;
    let mut lines = input.lines();
    while let (Some(line_1), Some(line_2), Some(line_3)) =
        (lines.next(), lines.next(), lines.next())
    {
        for item_1 in line_1.chars() {
            if line_2.contains(item_1) & line_3.contains(item_1) {
                priority = get_priority(item_1);
                break;
            }
        }
        answer += priority;
    }
    answer
}
