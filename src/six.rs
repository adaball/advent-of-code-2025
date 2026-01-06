use regex::Captures;
use regex::Regex;

fn get_input<'a>() -> &'a str {
    return include_str!("input/6.txt");
}

fn get_parts<'a>() -> Vec<Vec<&'a str>> {
    let lines: Vec<&str> = get_input()
        .split("\n")
        .filter(|a| a != &"")
        .collect();

    let mut parts: Vec<Vec<&'a str>> = Vec::new();
    for _ in 0..=4 {
        parts.push(Vec::new());
    }

    let num_re: Regex = Regex::new(r"(\d+(?:\s+)?)").unwrap();
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

    return parts;
}

fn perform_operation(parts: &Vec<Vec<&str>>, idx: usize) -> u64 {
    let n_1: u64 = parts[0][idx].replace(" ", "").parse().unwrap();
    let n_2: u64 = parts[1][idx].replace(" ", "").parse().unwrap();
    let n_3: u64 = parts[2][idx].replace(" ", "").parse().unwrap();
    let n_4: u64 = parts[3][idx].replace(" ", "").parse().unwrap();
    let op: &str = parts[4][idx];

    if op == "+" {
        return n_1 + n_2 + n_3 + n_4;
    } else {
        return n_1 * n_2 * n_3 * n_4;
    }
}

pub fn one() {
    let parts: Vec<Vec<&str>> = get_parts();
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

pub fn two() {
    let lines: Vec<&str> = get_input()
        .split("\n")
        .filter(|s| s != &"")
        .collect();

    let mut i: usize = 0;
    let mut op: &str = "";
    let mut numbers: Vec<u64> = Vec::new();
    let mut total: u64 = 0;
    while i < lines[0].len() {
        let line_1 = lines[0].get(i..i+1).unwrap();
        let line_2 = lines[1].get(i..i+1).unwrap();
        let line_3 = lines[2].get(i..i+1).unwrap();
        let line_4 = lines[3].get(i..i+1).unwrap();
        let line_5 = lines[4].get(i..i+1).unwrap();

        let combined: String = format!("{line_1}{line_2}{line_3}{line_4}").replace(" ", "");

        if line_5 != " " {
            op = line_5;
        }

        if combined != "" {
            numbers.push(combined.parse().unwrap());
        }

        if combined == "" || i == lines[0].len() - 1 {
            let mut sub_total: u64 = if op == "+" { 0 } else { 1 };
            for number in numbers {
                if op == "+" {
                    sub_total += number;
                } else {
                    sub_total *= number;
                }
            }
            total += sub_total;
            numbers = Vec::new();
        }

        i += 1;
    }

    println!("{total}");
}
