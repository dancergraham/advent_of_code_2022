use std::collections::HashSet;

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_part_1() {
        use super::*;
        let answer = part_1(TEST_INPUT);
        println!("The test answer to Part 1 is {}", answer);
        assert_eq!(answer, 7);
    }

    #[test]
    fn test_part_2() {
        use super::*;
        let answer = part_2(TEST_INPUT);
        println!("The test answer to Part 2 is {}", answer);
        assert_eq!(answer, 19);
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

fn part_1(input: &str) -> u32 {
    let mut answer = 0;
    for (i, vec) in input.as_bytes().windows(4).enumerate() {
        if vec.len() == vec.into_iter().collect::<HashSet<_>>().len() {
            answer = i + 4;
            break;
        }
    }
    return answer as u32;
}

fn part_2(input: &str) -> u32 {
    let mut answer = 0;
    for (i, vec) in input.as_bytes().windows(14).enumerate() {
        if vec.len() == vec.into_iter().collect::<HashSet<_>>().len() {
            answer = i + 14;
            break;
        }
    }
    return answer as u32;
}
