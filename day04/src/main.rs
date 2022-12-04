#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        let test_input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        use super::*;
        let answer = part_1(test_input);
        println!("The test answer to Part 1 is {}", answer);
        assert_eq!(answer, 2);
        let answer = part_2(test_input);
        println!("The test answer to Part 2 is {}", answer);
        assert_eq!(answer, 4);
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
            let answer_part_1 = part_1(&s);
            println!("The answer to Part 1 is {}", answer_part_1);
            let answer_part_2 = part_2(&s);
            println!("The answer to Part 2 is {}", answer_part_2); // 1000 is too high
        }
    }
}

#[derive(Copy, Clone)]
struct Range {
    start: u32,
    end: u32,
}

fn elf_to_range(elf: &str) -> Range {
    let values: Vec<&str> = elf.split('-').collect();
    return Range {
        start: values[0].parse::<u32>().unwrap(),
        end: values[1].parse::<u32>().unwrap(),
    };
}

// range 2 is entirely contained in range 1
fn contains(range_1: Range, range_2: Range) -> bool {
    return (range_2.start >= range_1.start) & (range_2.end <= range_1.end);
}

// range 2 is partially or fully contained in range 1
fn overlaps(range_1: Range, range_2: Range) -> bool {
    return !((range_2.end < range_1.start) | (range_2.start > range_1.end));
}

fn part_1(input: &str) -> i32 {
    let mut answer = 0;
    let lines = input.lines();
    for line in lines {
        let elves: Vec<&str> = line.split(',').collect();
        let elf_1 = elf_to_range(elves[0]);
        let elf_2 = elf_to_range(elves[1]);
        if contains(elf_1, elf_2) | contains(elf_2, elf_1) {
            answer = answer + 1
        }
    }
    return answer;
}

fn part_2(input: &str) -> i32 {
    let mut answer = 0;
    let lines = input.lines();
    for line in lines {
        let elves: Vec<&str> = line.split(',').collect();
        let elf_1 = elf_to_range(elves[0]);
        let elf_2 = elf_to_range(elves[1]);
        if overlaps(elf_1, elf_2) {
            answer = answer + 1
        }
    }
    return answer;
}
