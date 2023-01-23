use crate::Item::{Paper, Rock, Scissors};

fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    let test_input = "A Y
B X
C Z";
    let answer = part_1(test_input);
    println!("The test answer to Part 1 is {answer}");
    assert_eq!(answer, 15);

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
#[derive(Clone, Debug)]
struct Turn {
    choice: Item,
    score: i32,
}
#[derive(PartialEq, Clone, Debug)]
enum Item {
    Rock,
    Paper,
    Scissors,
}

fn my_turn(code: char) -> Turn {
    let mut result = Turn {
        choice: Scissors,
        score: 3,
    };
    if code == 'X' {
        result = Turn {
            choice: Rock,
            score: 1,
        };
    } else if code == 'Y' {
        result = Turn {
            choice: Paper,
            score: 2,
        };
    }
    result
}

fn my_turn_2(code: char, opponent: &Turn) -> Turn {
    let mut choice = Turn {
        choice: Scissors,
        score: 3,
    };
    if code == 'Y' {
        choice = opponent.clone();
    }

    if code == 'X' {
        // I need to lose
        if opponent.choice == Scissors {
            choice = Turn {
                choice: Paper,
                score: 2,
            };
        } else if opponent.choice == Paper {
            choice = Turn {
                choice: Rock,
                score: 1,
            };
        }
    } else if code == 'Z' {
        // I need to win
        if opponent.choice == Rock {
            choice = Turn {
                choice: Paper,
                score: 2,
            };
        } else if opponent.choice == Scissors {
            choice = Turn {
                choice: Rock,
                score: 1,
            };
        }
    }
    choice
}

fn outcome(my_choice: Item, opponent_choice: Item) -> i32 {
    let mut score = 0;
    if my_choice == opponent_choice {
        score = 3;
    } else if my_choice == Rock {
        if opponent_choice == Paper {
            score = 0
        }
        if opponent_choice == Scissors {
            score = 6
        }
    } else if my_choice == Paper {
        if opponent_choice == Rock {
            score = 6
        }
        if opponent_choice == Scissors {
            score = 0
        }
    } else if my_choice == Scissors {
        if opponent_choice == Rock {
            score = 0
        }
        if opponent_choice == Paper {
            score = 6
        }
    }
    score
}

fn opponent_turn(code: char) -> Turn {
    let mut result = Turn {
        choice: Scissors,
        score: 3,
    };
    if code == 'A' {
        result = Turn {
            choice: Rock,
            score: 1,
        };
    } else if code == 'B' {
        result = Turn {
            choice: Paper,
            score: 2,
        };
    }
    result
}

fn part_1(input: &str) -> i32 {
    let mut answer = 0;
    let lines = input.lines();
    for line in lines {
        let opponent = opponent_turn(line.chars().next().unwrap());
        let me = my_turn(line.chars().nth(2).unwrap());
        answer = answer + me.score + outcome(me.choice, opponent.choice)
    }
    answer
}

fn part_2(input: &str) -> i32 {
    let mut answer = 0;
    let lines = input.lines();
    for line in lines {
        let opponent = opponent_turn(line.chars().next().unwrap());
        let required_result = line.chars().nth(2).unwrap();
        let me = my_turn_2(required_result, &opponent);
        answer = answer + me.score + outcome(me.choice, opponent.choice)
    }
    answer
}
