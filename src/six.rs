use regex::Captures;
use regex::Regex;

fn get_input<'a>() -> &'a str {
    return include_str!("input/6.txt");
}

fn perform_operation(parts: &Vec<Vec<&str>>, idx: usize) -> u64 {
    let n_1: u64 = parts[0][idx].parse().unwrap();
    let n_2: u64 = parts[1][idx].parse().unwrap();
    let n_3: u64 = parts[2][idx].parse().unwrap();
    let n_4: u64 = parts[3][idx].parse().unwrap();
    let op: &str = parts[4][idx];

    if op == "+" {
        return n_1 + n_2 + n_3 + n_4;
    } else {
        return n_1 * n_2 * n_3 * n_4;
    }
}

pub fn one() {
    let lines: Vec<&str> = get_input()
        .split("\n")
        .filter(|a| a != &"")
        .collect();

    let mut parts: Vec<Vec<&str>> = Vec::new();
    for _ in 0..=4 {
        parts.push(Vec::new());
    }

    let num_re: Regex = Regex::new(r"(\d+)(?:\s+)?").unwrap();
    let op_re: Regex = Regex::new(r"([*+]){1}(?:\s+)?").unwrap();

    for i in 0..=4 {
        let re: &Regex;
        if i < 4 {
            re = &num_re;
        } else {
            re = &op_re;
        }

        let caps: Vec<Captures> = re.captures_iter(lines[i]).collect();
        for cap in caps {
            let (_, [s]) = cap.extract();
            parts[i].push(s);
        }
    }

    let mut answers: Vec<u64> = Vec::new();
    for i in 0..parts[0].len() {
        answers.push(perform_operation(&parts, i));
    }

    let mut total = 0;
    for answer in answers {
        total += answer;
    }

    println!("{total}");
}
