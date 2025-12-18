fn get_input<'a>() -> &'a str {
    return include_str!("input/3.txt");
}

fn combine(a: &str, b: &str) -> u32 {
    let combined = format!("{}{}", a, b);

    return combined.parse().unwrap();
}

fn get_highest(line: &str) -> u32 {
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
    let lines: Vec<&str> = get_input().split("\n").collect();
    let mut total = 0;
    
    for line in lines {
        total += get_highest(line);
    }

    println!("{total}");
}
