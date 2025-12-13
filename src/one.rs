use regex::Regex;

fn get_input<'a>() -> &'a str {
    return include_str!("input/1.txt");
}

fn get_rotations<'a>() -> Vec<(&'a str, &'a str)> {
    let re = Regex::new("(L|R)(\\d+)").unwrap();
    let input_str = get_input();
    let lines: Vec<&str> = input_str.split('\n').collect();
    let mut rotations: Vec<(&str, &str)> = Vec::new();

    for line in lines {
        let res_opt = re.captures(line);

        if res_opt.is_none() {
            break;
        }

        let res = res_opt.unwrap();
        let dir = res.get(1).unwrap().as_str();
        let amt = res.get(2).unwrap().as_str();
        rotations.push((dir, amt));
    }

    return rotations;
}

pub fn one() {
    let rotations: Vec<(&str, &str)> = get_rotations();
    let mut num = 50;
    let mut zeros_found = 0;

    for rotation in rotations {
        let mut amt: i32 = rotation.1.parse().unwrap();

        while amt > 0 {
            if rotation.0 == "L" {
                num -= 1;
            } else {
                num += 1;
            }

            if num == -1 {
                num = 99;
            } else if num == 100 {
                num = 0;
            }

            amt -= 1;
        }

        if num == 0 {
            zeros_found += 1;
        }
    }

    println!("{}", zeros_found);
}

pub fn two() {
    let rotations: Vec<(&str, &str)> = get_rotations();
    let mut num = 50;
    let mut zeros_found = 0;

    for rotation in rotations {
        let mut amt: i32 = rotation.1.parse().unwrap();

        while amt > 0 {
            if rotation.0 == "L" {
                num -= 1;
            } else {
                num += 1;
            }

            if num == -1 {
                num = 99;
            } else if num == 100 {
                num = 0;
            }

            if num == 0 {
                zeros_found += 1;
            }

            amt -= 1;
        }
    }

    println!("{}", zeros_found);
}
