fn get_input<'a>() -> &'a str {
    return include_str!("input/5.txt");
}

fn in_range(id: u64, ranges: &Vec<(u64, u64)>) -> bool {
    for range in ranges {
        if id >= range.0 && id <= range.1 {
            return true;
        }
    }

    return false;
}

pub fn one() {
    let input: Vec<&str> = get_input()
        .split("\n")
        .collect();

    let mut i = 0;
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    loop {
        let line: &str = input[i];

        if line == "" {
            i += 1;
            break;
        }

        let mut split = line.split("-");
        let lower: &str = split.next().unwrap();
        let upper: &str = split.next().unwrap();

        ranges.push((lower.parse().unwrap(), upper.parse().unwrap()));

        i += 1;
    }

    let mut fresh_count = 0;

    loop {
        let line: &str = input[i];

        if line == "" {
            break;
        }

        let id: u64 = line.parse().unwrap();

        if in_range(id, &ranges) {
            fresh_count += 1;
        }

        i += 1;
    }

    println!("{fresh_count}");
}
