fn all_chars_same(s: &str) -> bool {
    let first = s.get(0..1).unwrap();
    let mut all_equal = false;

    for i in 1..s.len() {
        all_equal = first == s.get(i..i+1).unwrap();

        if !all_equal {
            break;
        }
    }

    return all_equal;
}

fn all_strs_same(strs: &Vec<&str>) -> bool {
    let first = strs[0];

    for s in strs {
        if *s != first {
            return false;
        }
    }

    return true;
}

fn get_digits(num: f64) -> Vec<f64> {
    let base: f64 = 10.0;
    let mut exp = 0.0;

    while base.powf(exp) <= num {
        exp += 1.0;
    }

    exp -= 1.0;

    let mut digits: Vec<f64> = Vec::new();
    let mut num_mut = num;
    while exp >= 0.0 {
        let frac: f64 = num_mut / base.powf(exp);
        let digit = frac.floor();
        digits.push(digit);

        num_mut = num_mut - digit * base.powf(exp);
        exp -= 1.0;
    }

    return digits;
}

fn get_input<'a>() -> &'a str {
    return include_str!("input/2.txt");
}

fn get_limits() -> Vec<(u64, u64)> {
    let input = get_input();
    let ranges: Vec<&str> = input.split(',').collect();
    let mut limits: Vec<(u64, u64)> = Vec::new();

    for range in ranges {
        let str_limits: Vec<&str> = range.trim_end().split('-').collect();
        let lower = str_limits[0].parse().unwrap();
        let upper = str_limits[1].parse().unwrap();

        limits.push((lower, upper));
    }

    return limits;
}

fn is_invalid_one(id: f64) -> bool {
    let digits = get_digits(id);

    if digits.len() % 2 != 0 {
        return false;
    }

    let mut a = digits.len() / 2 - 1;
    let mut b = digits.len() - 1;

    loop {
        if digits[a] != digits[b] {
            return false;
        }

        if a == 0 {
            break;
        }

        a -= 1;
        b -= 1;
    }


    return true;
}

fn is_invalid_two(id: u64) -> bool {
    let id_str = id.to_string();

    if all_chars_same(&id_str) {
        return true;
    }

    let start = 0;
    let mut size = 2;

    while start + size < id_str.len() {
        let mut inner_start = start;
        let mut total_length = 0;
        let mut sub_strs: Vec<&str> = Vec::new();

        while inner_start + size <= id_str.len() {
            let sub_str = id_str.get(inner_start..inner_start + size).unwrap();

            sub_strs.push(sub_str);
            total_length += size;
            inner_start += size;
        }

        size += 1;

        if total_length == id_str.len() && all_strs_same(&sub_strs) {
            return true;
        }

    }

    return false;
}

pub fn one() {
    let limits: Vec<(u64, u64)> = get_limits();
    let mut total = 0;

    for limit in limits {
        let mut num = limit.0;
        while num <= limit.1 {
            if is_invalid_one(num as f64) {
                total += num;
            }

            num += 1;
        }
    }

    println!("{total}");
}

pub fn two() {
    let limits: Vec<(u64, u64)> = get_limits();
    let mut total = 0;

    for limit in limits {
        let mut num = limit.0;
        while num <= limit.1 {
            if is_invalid_two(num) {
                total += num;
            }

            num += 1;
        }
    }

    println!("{total}");
}
