fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    let path = Path::new("input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {display}: {why}"),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {display}: {why}"),
        Ok(_) => {
            let answer_part_1 = part_1(&s);
            println!("The answer to Part 1 is {answer_part_1}");
            let answer_part_2 = part_2(&s);
            println!("The answer to Part 2 is {answer_part_2}");
        }
    }
}

fn part_1(input: &str) -> i32 {
    let mut answer = 0;
    let mut elf = 0;
    let lines = input.lines();
    for line in lines {
        if line == "" {
            if elf > answer {
                answer = elf;
            }
            elf = 0;
        } else {
            let calories = line.parse::<i32>().unwrap();
            elf = elf + calories;
        }
    }
    return answer;
}

fn part_2(input: &str) -> i32 {
    let mut largest = 0;
    let mut second = 0;
    let mut third = 0;
    let mut elf = 0;
    let lines = input.lines();
    for line in lines {
        if line == "" {
            if elf > third {
                third = elf;
                if third > second {
                    (second, third) = (third, second);
                    if second > largest {
                        (second, largest) = (largest, second);
                    }
                }
            }
            elf = 0;
        } else {
            let calories = line.parse::<i32>().unwrap();
            elf = elf + calories;
        }
    }
    return largest + second + third;
}
