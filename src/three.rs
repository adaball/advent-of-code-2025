fn get_input<'a>() -> &'a str {
    return include_str!("input/3.txt");
}

fn combine(a: &str, b: &str) -> u32 {
    let combined = format!("{}{}", a, b);

    return combined.parse().unwrap();
}

fn largest_n_digit_num(start_idx: u32, digit_size: u32, bank: &str) -> u64 {
    let mut largest: u64 = 0;
    let mut largest_idx: u32 = 0;
    let bank_len: u32 = bank.len() as u32;

    for i in start_idx..(bank_len - digit_size + 1) {
        let idx: usize = i as usize;
        let v: u64 = bank[idx..idx+1].parse().unwrap();

        if v > largest {
            largest = v;
            largest_idx = i;
        }
    }

    if digit_size > 1 {
        return largest * 10u64.pow(digit_size - 1) + largest_n_digit_num(largest_idx + 1, digit_size - 1, bank);
    } else {
        return largest;
    }
}

fn largest_two_digit_num(line: &str) -> u32 {
    let mut highest: u32 = 0;

    for i in 0..line.len() {
        for j in i+1..line.len() {
            let a = line.get(i..i+1).unwrap();
            let b = line.get(j..j+1).unwrap();
            let num = combine(&a, &b);

            if num > highest {
                highest = num;
            }
        }
    }

    return highest;
}

pub fn one() {
    let lines: Vec<&str> = get_input()
        .split("\n")
        .filter(|s| *s != "")
        .collect();
    let mut total = 0;
    
    for line in lines {
        total += largest_two_digit_num(line);
    }

    println!("{total}");
}

pub fn two() {
    let lines: Vec<&str> = get_input()
        .split("\n")
        .filter(|s| *s != "")
        .collect();
    let mut total: u64 = 0;

    for line in lines {
        total += largest_n_digit_num(0, 12, line);
    }

    println!("{total}");
}
