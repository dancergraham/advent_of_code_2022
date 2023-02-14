use std::collections::HashSet;

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_part_1() {
        use super::*;
        let answer = part_1(TEST_INPUT);
        println!("The test answer to Part 1 is {}", answer);
        assert_eq!(answer, 13);
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

fn move_tail(head: (isize, isize), mut tail: (isize, isize)) -> (isize, isize) {
    let distance = (head.0 - tail.0, head.1 - tail.1);
    if (distance.0 == 2) | (distance.0 == -2) | (distance.1 == 2) | (distance.1 == -2) {
        if distance.0 != 0 {
            tail.0 = tail.0 + distance.0 / distance.0.abs_diff(0) as isize
        }
        if distance.1 != 0 {
            tail.1 = tail.1 + distance.1 / distance.1.abs_diff(0) as isize
        }
    }
    return tail;
}
fn move_head(mut point: (isize, isize), direction: &str, distance: isize) -> (isize, isize) {
    if direction == "R" {
        point = (point.0 + distance, point.1)
    }
    if direction == "L" {
        point = (point.0 - distance, point.1)
    }
    if direction == "U" {
        point = (point.0, point.1 + distance)
    }
    if direction == "D" {
        point = (point.0, point.1 - distance)
    }
    return point;
}

fn part_1(input: &str) -> isize {
    let mut head_point = (0, 0);
    let mut tail_point = (0, 0);
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for line in input.lines() {
        let values: Vec<&str> = line.split(' ').collect();
        let direction = values[0];
        let distance = values[1].parse::<isize>().expect("Always 2 values");
        for _i in 0..distance {
            head_point = move_head(head_point, direction, 1);
            tail_point = move_tail(head_point, tail_point);
            visited.insert(tail_point);
        }
    }
    return visited.len() as isize;
}

fn part_2(input: &str) -> isize {
    let mut snake: [(isize, isize); 10] = [(0, 0); 10];
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for line in input.lines() {
        let values: Vec<&str> = line.split(' ').collect();
        let direction = values[0];
        let distance = values[1].parse::<isize>().expect("Always 2 values");
        for _i in 0..distance {
            snake[0] = move_head(snake[0], direction, 1);
            for j in 1..10 {
                snake[j] = move_tail(snake[j - 1], snake[j])
            }
            visited.insert(snake[9]);
        }
    }
    return visited.len() as isize;
}
