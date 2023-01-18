fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    let test_input = "A Y
B X
C Z";
    let answer = part_1(test_input);
    println!("The test answer to Part 1 is {}", answer);
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
    choice: String,
    score: i32,
}

fn my_turn(code: char) -> Turn {
    let mut result = Turn {
        choice: "SCISSORS".to_string(),
        score: 3,
    };
    if code == 'X' {
        result = Turn {
            choice: "ROCK".to_string(),
            score: 1,
        };
    } else if code == 'Y' {
        result = Turn {
            choice: "PAPER".to_string(),
            score: 2,
        };
    }
    result
}

fn my_turn_2(code: char, opponent: Turn) -> Turn {
    let mut choice = Turn {
        choice: "SCISSORS".to_string(),
        score: 3,
    };
    if code == 'Y' {
        choice = opponent.clone();
    }

    if code == 'X' {
        // I need to lose
        if opponent.choice == "SCISSORS" {
            choice = Turn {
                choice: "PAPER".to_string(),
                score: 2,
            };
        } else if opponent.choice == "PAPER" {
            choice = Turn {
                choice: "ROCK".to_string(),
                score: 1,
            };
        }
    } else if code == 'Z' {
        // I need to win
        if opponent.choice == "ROCK" {
            choice = Turn {
                choice: "PAPER".to_string(),
                score: 2,
            };
        } else if opponent.choice == "SCISSORS" {
            choice = Turn {
                choice: "ROCK".to_string(),
                score: 1,
            };
        }
    }

    choice
}

fn outcome(my_choice: String, opponent_choice: String) -> i32 {
    let mut score = 0;
    if my_choice == opponent_choice {
        score = 3;
    } else if my_choice == "ROCK" {
        if opponent_choice == "PAPER" {
            score = 0
        }
        if opponent_choice == "SCISSORS" {
            score = 6
        }
    } else if my_choice == "PAPER" {
        if opponent_choice == "ROCK" {
            score = 6
        }
        if opponent_choice == "SCISSORS" {
            score = 0
        }
    } else if my_choice == "SCISSORS" {
        if opponent_choice == "ROCK" {
            score = 0
        }
        if opponent_choice == "PAPER" {
            score = 6
        }
    }
    score
}

fn opponent_turn(code: char) -> Turn {
    let mut result = Turn {
        choice: "SCISSORS".to_string(),
        score: 3,
    };
    if code == 'A' {
        result = Turn {
            choice: "ROCK".to_string(),
            score: 1,
        };
    } else if code == 'B' {
        result = Turn {
            choice: "PAPER".to_string(),
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

        let me = my_turn_2(required_result, opponent.clone());
        answer = answer + me.score + outcome(me.choice, opponent.choice)
    }
    answer
}
