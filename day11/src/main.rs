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

    #[test]
    fn test_part_2() {
        use super::*;
        let answer = part_2(TEST_INPUT);
        println!("The test answer to Part 2 is {}", answer);
        assert_eq!(answer, 2713310158);
    }
}

struct Monkey {
    items: Vec<u128>,
    operation: String,
    worry_level: u128,
    operations: u128,
    test_divisible: u128,
    true_monkey: u128,
    false_monkey: u128,
}
fn parse(s: &str) -> u128 {
    s.parse::<u128>().unwrap()
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
    let mut amount: u128 = 0;
    if operation_complement == "old" {
        operation = String::from("**");
    } else {
        amount = parse(operations[1]) as u128;
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
        worry_level: amount,
        operations: 0,
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
            println!("The answer to Part 2 is {}", part_2(&s));
        }
    }
}

fn operate(item: &u128, op: &String, amount: u128, max_worry_level: u128) -> u128 {
    return if op == "**" {
        item * item % max_worry_level
    } else if op == "*" {
        item * amount % max_worry_level
    } else {
        item + amount % max_worry_level
    };
}

fn part_1(input: &str) -> u128 {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(parse_monkey).collect();
    for _turn in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let true_monkey = monkey.true_monkey.clone() as usize;
            let false_monkey = monkey.false_monkey.clone() as usize;
            let mut true_items = Vec::new();
            let mut false_items = Vec::new();

            for item in &monkey.items {
                monkey.operations += 1;
                let new_item = &operate(item, &monkey.operation, monkey.worry_level, 10_000_000);
                let new_item = &(new_item / 3);
                if &(new_item % monkey.test_divisible) == &u128::from(0) {
                    true_items.push(*new_item)
                } else {
                    false_items.push(*new_item)
                }
            }
            monkey.items.clear();
            let monkey = &mut monkeys[true_monkey];
            monkey.items.append(&mut true_items);
            let monkey = &mut monkeys[false_monkey];
            monkey.items.append(&mut false_items);
        }
    }
    let mut most_active = 0;
    let mut next_most = 0;
    for monkey in monkeys {
        if monkey.operations > next_most {
            next_most = monkey.operations;
            if next_most > most_active {
                (most_active, next_most) = (next_most, most_active)
            }
        }
    }
    return most_active * next_most;
}

fn part_2(input: &str) -> u128 {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(parse_monkey).collect();
    let mut maximum_worry = 1;
    for monkey in &monkeys {
        maximum_worry *= monkey.test_divisible;
    }
    for _turn in 0..5 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let true_monkey = monkey.true_monkey.clone() as usize;
            let false_monkey = monkey.false_monkey.clone() as usize;
            let mut true_items = Vec::new();
            let mut false_items = Vec::new();

            for item in &monkey.items {
                monkey.operations += 1;
                let new_item = &operate(item, &monkey.operation, monkey.worry_level, maximum_worry);
                if &(new_item % monkey.test_divisible) == &0 as &u128 {
                    true_items.push(*new_item)
                } else {
                    false_items.push(*new_item)
                }
            }
            monkey.items.clear();
            let monkey = &mut monkeys[true_monkey];
            monkey.items.append(&mut true_items);
            let monkey = &mut monkeys[false_monkey];
            monkey.items.append(&mut false_items);
        }
    }
    let mut most_active = 0;
    let mut next_most = 0;
    for monkey in monkeys {
        if monkey.operations > next_most {
            next_most = monkey.operations;
            if next_most > most_active {
                (most_active, next_most) = (next_most, most_active)
            }
        }
    }
    return most_active * next_most;
}
