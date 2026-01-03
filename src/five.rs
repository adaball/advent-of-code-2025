use std::cmp;

fn combine_all_ranges(ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut ranges_iter = ranges.into_iter();
    let mut result: Vec<(u64, u64)> = Vec::new();

    let mut curr = ranges_iter.next();
    loop {
        if curr.is_none() {
            break;
        }

        let next = ranges_iter.next();

        if next.is_none() {
            result.push(curr.unwrap());
            break;
        }

        if overlap(curr.unwrap(), next.unwrap()) {
            curr = Some(combine_ranges(curr.unwrap(), next.unwrap()));
        } else {
            result.push(curr.unwrap());
            curr = next;
        }
    }

    return result;
}

fn combine_ranges(a: (u64, u64), b: (u64, u64)) -> (u64, u64) {
    let min = cmp::min(a.0, b.0);
    let max = cmp::max(a.1, b.1);

    return (min, max);
}

fn get_input<'a>() -> &'a str {
    return include_str!("input/5.txt");
}

fn get_ranges(input: &Vec<&str>) -> Vec<(u64, u64)> {
    let mut i = 0;
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    loop {
        let line: &str = input[i];

        if line == "" {
            break;
        }

        let mut split = line.split("-");
        let lower: &str = split.next().unwrap();
        let upper: &str = split.next().unwrap();

        ranges.push((lower.parse().unwrap(), upper.parse().unwrap()));

        i += 1;
    }

    return ranges;
}

fn in_range(id: u64, ranges: &Vec<(u64, u64)>) -> bool {
    for range in ranges {
        if id >= range.0 && id <= range.1 {
            return true;
        }
    }

    return false;
}

fn overlap(a: (u64, u64), b: (u64, u64)) -> bool {
    if a.0 == b.0 && a.1 == b.1 {
        return true;
    } else if a.0 < b.1 && a.1 > b.0 {
        return true;
    } else if a.0 == b.1 || a.1 == b.0 {
        return true;
    }

    return false;
}

pub fn one() {
    let input: Vec<&str> = get_input()
        .split("\n")
        .collect();

    let ranges: Vec<(u64, u64)> = get_ranges(&input);
    let mut i = 0;
    loop {
        let line: &str = input[i];

        if line == "" {
            i += 1;
            break;
        }

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

pub fn two() {
    let input: Vec<&str> = get_input()
        .split("\n")
        .collect();

    let mut ranges: Vec<(u64, u64)> = get_ranges(&input);
    ranges.sort_by(|a, b| a.0.cmp(&b.1));

    let mut range_cnt = ranges.len();
    loop {
        ranges = combine_all_ranges(ranges);

        if ranges.len() == range_cnt {
            break;
        }

        range_cnt = ranges.len();
    }

    let mut total_ids = 0;
    for range in ranges {
        total_ids += range.1 - range.0 + 1;
    }

    println!("{total_ids}");
}
