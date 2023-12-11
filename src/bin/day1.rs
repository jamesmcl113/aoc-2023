const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let puzzle_lines: u32 = std::fs::read_to_string("../input/day1.txt")
        .unwrap()
        .lines()
        .map(|l| get_digits(&l.chars().collect::<Vec<_>>()))
        .sum();

    println!("{puzzle_lines}");
}

fn parse_number(line: &[char]) -> Option<(u32, &[char])> {
    for (i, num) in NUMBERS.iter().enumerate() {
        let num_chars = num.chars().collect::<Vec<_>>();
        if line.starts_with(&num_chars) {
            let number_len = num_chars.len();
            return Some((i as u32 + 1, &line[number_len - 1..]));
        }
    }

    None
}

fn parse_digit(line: &[char]) -> Option<(u32, &[char])> {
    line.first()
        .and_then(|ch| ch.to_digit(10))
        .map(|u| (u, &line[1..]))
}

fn get_digits(line: &[char]) -> u32 {
    let mut chars = line;
    let mut digits: Vec<u32> = vec![];

    loop {
        if chars.is_empty() {
            break;
        }

        if let Some((digit, rest)) = parse_digit(&chars) {
            digits.push(digit);
            chars = rest;
        } else if let Some((digit, rest)) = parse_number(&chars) {
            digits.push(digit);
            chars = rest;
        } else {
            chars = &chars[1..];
        }
    }

    let first = digits.first().unwrap();
    if digits.len() > 1 {
        let last = digits.last().unwrap();
        first * 10 + last
    } else {
        first * 10 + first
    }
}

#[cfg(test)]
mod test {
    use crate::get_digits;

    #[test]
    fn test() {
        let line = "oneight".chars().collect::<Vec<_>>();
        assert_eq!(get_digits(&line), 18);
    }
}
