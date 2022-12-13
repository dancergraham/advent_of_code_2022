#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part_1() {
        use super::*;
        let answer = part_1(TEST_INPUT);
        println!("The test answer to Part 1 is {}", answer);
        assert_eq!(answer, 10605);
    }
}

struct Monkey {
    items: Vec<i32>,
    operation: String,
    amount: i32,
    test_divisible: i32,
    true_monkey: i32,
    false_monkey: i32,
}
fn parse(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}
fn parse_monkey(text: &str) -> Monkey {
    let mut lines = text.lines();
    lines.next(); // header Monkey #
    let items = lines
        .next()
        .expect("I'm feeling lucky")
        .split("items: ")
        .collect::<Vec<&str>>()[1]
        .split(", ")
        .map(parse)
        .collect();
    let operations: Vec<&str> = lines
        .next()
        .expect("I'm feeling lucky")
        .split("new = old ")
        .collect::<Vec<&str>>()[1]
        .split(" ")
        .collect();
    let mut operation = operations[0].parse().unwrap();
    let operation_complement = operations[1];
    let mut amount = 0;
    if operation_complement == "old" {
        operation = String::from("**");
    } else {
        amount = parse(operations[1]);
    }
    let test_divisible = parse(
        lines
            .next()
            .expect("I'm feeling lucky")
            .split("divisible by ")
            .collect::<Vec<&str>>()[1],
    );
    let true_monkey = parse(
        lines
            .next()
            .expect("I'm feeling lucky")
            .split("true: throw to monkey ")
            .collect::<Vec<&str>>()[1],
    );
    let false_monkey = parse(
        lines
            .next()
            .expect("I'm feeling lucky")
            .split("false: throw to monkey ")
            .collect::<Vec<&str>>()[1],
    );
    return Monkey {
        items,
        operation,
        amount,
        test_divisible,
        true_monkey,
        false_monkey,
    };
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
            // println!("The answer to Part 2 is {}", part_2(&s));
        }
    }
}

fn operate(item: &i32, op: &String, amount: i32) -> i32 {
    return if op == "**" {
        item * item
    } else if op == "*" {
        item * amount
    } else {
        item + amount
    };
}

fn part_1(input: &str) -> i32 {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(parse_monkey).collect();
    for _turn in 0..20 {
        for m_i in 0..monkeys.len() {
            let mut monkey = &monkeys[m_i];

            for mut item in &monkey.items {
                let mut new_item = &operate(item, &monkey.operation, monkey.amount);
                let new_item = &(new_item / 3);
                if &(new_item % monkey.test_divisible) == &0 as &i32 {
                    monkeys[monkey.true_monkey as usize].items.push(*new_item)
                } else {
                    monkeys[monkey.false_monkey as usize].items.push(*new_item)
                }
            }
            monkey.items.clear();
        }
    }
    return 0;
}
