fn get_input<'a>() -> &'a str {
    return include_str!("input/2.txt");
}

fn is_invalid(id: f64) -> bool {
    let base: f64 = 10.0;
    let mut exp = 0.0;
    let mut res = base.powf(exp);

    while res <= id {
        exp += 1.0;
        res = base.powf(exp);
    }

    exp -= 1.0;

    let mut digits: Vec<f64> = Vec::new();
    let mut id_mut = id;
    while exp >= 0.0 {
        let frac: f64 = id_mut / base.powf(exp);
        let digit = frac.floor();
        digits.push(digit);

        id_mut = id_mut - digit * base.powf(exp);
        exp -= 1.0;
    }

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

pub fn one() {
    let input = get_input();
    let ranges: Vec<&str> = input.split(',').collect();
    let mut limits: Vec<(u64, u64)> = Vec::new();

    for range in ranges {
        let str_limits: Vec<&str> = range.trim_end().split('-').collect();
        let lower = str_limits[0].parse().unwrap();
        let upper = str_limits[1].parse().unwrap();

        limits.push((lower, upper));
    }

    let mut total = 0;
    for limit in limits {
        let mut num = limit.0;
        while num <= limit.1 {
            if is_invalid(num as f64) {
                total += num;
            }

            num += 1;
        }
    }

    println!("{total}");
}
