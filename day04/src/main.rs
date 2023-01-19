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
        println!("The test answer to Part 1 is {answer}");
        assert_eq!(answer, 2);
        let answer = part_2(test_input);
        println!("The test answer to Part 2 is {answer}");
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
    let mut file = match File::open(path) {
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

#[derive(Copy, Clone)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    // range 2 is entirely contained in range 1
    fn contains(self, other: Range) -> bool {
        (other.start >= self.start) & (other.end <= self.end)
    }

    // range 2 is partially or fully contained in range 1
    fn overlaps(self, other: Range) -> bool {
        !((other.end < self.start) | (other.start > self.end))
    }
}
fn elf_to_range(elf: &str) -> Range {
    let values: Vec<&str> = elf.split('-').collect();
    Range {
        start: values[0].parse::<u32>().unwrap(),
        end: values[1].parse::<u32>().unwrap(),
    }
}

fn part_1(input: &str) -> i32 {
    let mut answer = 0;
    let lines = input.lines();
    for line in lines {
        let elves: Vec<&str> = line.split(',').collect();
        let elf_1 = elf_to_range(elves[0]);
        let elf_2 = elf_to_range(elves[1]);
        if elf_1.contains(elf_2) | elf_2.contains(elf_1) {
            answer += 1
        }
    }
    answer
}

fn part_2(input: &str) -> i32 {
    let mut answer = 0;
    let lines = input.lines();
    for line in lines {
        let elves: Vec<&str> = line.split(',').collect();
        let elf_1 = elf_to_range(elves[0]);
        let elf_2 = elf_to_range(elves[1]);
        if elf_1.overlaps(elf_2) {
            answer += 1
        }
    }
    answer
}
