pub fn solve() {
    let input = std::fs::read_to_string("input/day_01.txt").unwrap();
    println!("Day 1, part 1: {}", part1(input.clone()));
    println!("Day 1, part 2: {}", part2(input));
}

fn part1(input: String) -> u32 {
    let lines = input.lines();
    let mut sum: u32 = 0;
    let mut lines_numbers: Vec<Vec<u32>> = Vec::new();
    for (idx, line) in lines.enumerate() {
        lines_numbers.push(Vec::new());
        for char in line.chars() {
            if char.is_numeric() {
                lines_numbers[idx].push(char.to_digit(10).unwrap());
            }
        }
    }
    for line in lines_numbers {
        sum += append_numbers(vec![line[0], line[line.len() - 1]]);
    }
    sum
}

fn part2(input: String) -> u32 {
    let lines = input.lines();
    let mut sum: u32 = 0;
    let mut lines_numbers: Vec<Vec<u32>> = Vec::new();
    for (idx, line) in lines.enumerate() {
        lines_numbers.push(Vec::new());
        let mut buffer = String::new();
        for ch in line.chars() {
            buffer.push(ch);
            if ch.is_numeric() {
                lines_numbers[idx].push(ch.to_digit(10).unwrap());
            } else {
                let r = contains_number_literal(buffer.clone());
                if !r.is_empty() {
                    let number = number_literal_to_number(r.clone());
                    if let Some(s) = number {
                        lines_numbers[idx].push(s);
                    }
                    buffer = format!("{ch}");
                }
            }
        }
    }
    for line in lines_numbers {
        sum += append_numbers(vec![line[0], line[line.len() - 1]]);
    }
    sum
}

fn append_numbers(numbers: Vec<u32>) -> u32 {
    let mut buff = String::new();
    for number in numbers {
        buff.push_str(&number.to_string());
    }
    buff.parse::<u32>().unwrap()
}

fn number_literal_to_number(input: String) -> Option<u32> {
    match input.as_str() {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn contains_number_literal(input: String) -> String {
    if input.contains("one") {
        return "one".to_string();
    } else if input.contains("two") {
        return "two".to_string();
    } else if input.contains("three") {
        return "three".to_string();
    } else if input.contains("four") {
        return "four".to_string();
    } else if input.contains("five") {
        return "five".to_string();
    } else if input.contains("six") {
        return "six".to_string();
    } else if input.contains("seven") {
        return "seven".to_string();
    } else if input.contains("eight") {
        return "eight".to_string();
    } else if input.contains("nine") {
        return "nine".to_string();
    } else if input.contains("zero") {
        return "zero".to_string();
    }
    "".to_string()
}
