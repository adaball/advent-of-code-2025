use std::fs;

fn display(grid: &Vec<Vec<&str>>, x: usize, y: usize) {
    let mut display_str: String = String::from("");

    if y != 0 {
        if x != 0 {
            display_str = format!("{display_str}{}", grid[x-1][y-1]);
        }
        display_str = format!("{display_str}{}", grid[x][y-1]);
        if x != grid.len() - 1 {
            display_str = format!("{display_str}{}\n", grid[x+1][y-1]);
        }
    }

    if x != 0 {
        display_str = format!("{display_str}{}", grid[x-1][y]);
    }

    display_str = format!("{display_str}{}", grid[x][y]);

    if x != grid.len() - 1 {
        display_str = format!("{display_str}{}\n", grid[x+1][y]);
    }
    
    if y != grid[0].len() - 1 {
        if x != 0 {
            display_str = format!("{display_str}{}", grid[x-1][y+1]);
        }
        display_str = format!("{display_str}{}", grid[x][y+1]);
        if x != grid.len() - 1 {
            display_str = format!("{display_str}{}", grid[x+1][y+1]);
        }
    }

    println!("{display_str}");
}

fn get_input<'a>() -> &'a str {
    return include_str!("input/7.txt");
}

fn out(grid: &Vec<Vec<&str>>) {
    let mut out_str: String = String::from("");
    for y in 0..grid[0].len() {
        for x in 0..grid.len() {
            out_str = format!("{}{}", out_str, grid[x][y]);
        }
        out_str = format!("{}\n", out_str);
    }

    fs::write("foo.txt", out_str).expect("Should be able to write");
}

pub fn one() {
    let lines: Vec<&str> = get_input()
        .split("\n")
        .filter(|s| s != &"")
        .collect();

    let mut grid: Vec<Vec<&str>> = Vec::new();
    let x_max = lines[0].len();
    let y_max = lines.len();

    for x in 0..x_max {
        grid.push(Vec::new());
        for y in 0..y_max {
            grid[x].push(lines[y].get(x..x+1).unwrap());
        }
    }

    // find S
    let mut start_coords: (usize, usize) = (0, 0);
    for x in 0..x_max {
        for y in 0..y_max {
            if grid[x][y] == "S" {
                start_coords = (x, y);
                break;
            }
        }
    }

    // assert cell below start_coords is '.' and replace it with '|'
    assert_eq!(".", grid[start_coords.0][start_coords.1+1]);
    grid[start_coords.0][start_coords.1+1] = "|";

    let mut beam_split_count = 0;
    for y in start_coords.1+2..y_max {
        for x in 0..x_max {
            if grid[x][y] == "." && grid[x][y-1] == "|" {
                grid[x][y] = "|";
            } else if grid[x][y] == "^" && grid[x][y-1] == "|" {
                grid[x-1][y] = "|";
                grid[x+1][y] = "|";

                beam_split_count += 1
            }
        }
    }

    println!("{beam_split_count}");
}
