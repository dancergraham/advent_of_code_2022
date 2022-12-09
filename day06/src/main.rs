use std::collections::HashSet;
use std::fs::File;

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part_1() {
        use super::*;
        let answer = part_1(TEST_INPUT);
        println!("The test answer to Part 1 is {}", answer);
        assert_eq!(answer, 95437);
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
struct File_obj<'a> {
    name: &'a str,
    size: usize,
    contents: Vec<File_obj<'a>>,
    is_dir: bool,
}

fn part_1(input: &str) -> u32 {
    use slab_tree::*;
    let mut answer = 0;
    let mut cd = File_obj {
        name: &"/",
        size: 0,
        contents: Vec::new(),
        is_dir: true,
    };
    let mut path = vec![&cd];
    for line in input.lines() {
        if line.chars().next().expect("no blank lines") == '$' {
            line.chars().next();
            let commands: Vec<&str> = line.split(' ').collect();
            if commands[1] == "ls" {
                continue;
            } else if commands[1] == "cd" {
                if commands[2] == "/" {
                    continue;
                } else if commands[2] == ".." {
                    let index = path.len();
                    path.remove(index);
                }
            }
            println!("{}", commands[1])
        } else {
            let values: Vec<&str> = line.split(' ').collect();
            if values[0].parse::<usize>().is_ok() {
                cd.contents.push(File_obj {
                    name: values[1],
                    size: values[0].parse::<usize>().expect("OK"),
                    contents: vec![],
                    is_dir: false,
                })
            } else {
                cd.contents.push(File_obj {
                    name: values[1],
                    size: 0,
                    contents: vec![],
                    is_dir: true,
                })
            }
            println!("{}", line)
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
