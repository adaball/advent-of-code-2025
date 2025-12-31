fn get_input<'a>() -> &'a str {
    return include_str!("input/4.txt");
}

fn get_surrounding<'a>(grid: &'a Vec<Vec<&'a str>>, x: i32, y: i32) -> Vec<Option<&'a str>> {
    let mut surrounding: Vec<Option<&str>> = Vec::new();

    // adjust x and y by:
    //  x    y   dir
    // -1 , -1   NW
    //  0 , -1   N
    //  1 , -1   NE
    // -1 ,  0   W
    //  1 ,  0   E
    // -1 ,  1   SW
    //  0 ,  1   S
    //  1 ,  1   SE

    let x_u: usize = x as usize;
    let y_u: usize = y as usize;
    let max_x = grid.len() as i32 - 1;
    let max_y = grid[0].len() as i32 - 1;

    // NW
    if x - 1 < 0 || y - 1 < 0 {
        surrounding.push(None);
    } else {
        surrounding.push(Some(grid[x_u-1][y_u-1]));
    }

    // N
    if y - 1 < 0 {
        surrounding.push(None);
    } else {
        surrounding.push(Some(grid[x_u][y_u-1]));
    }

    // NE
    if x + 1 > max_x || y - 1 < 0 {
        surrounding.push(None);
    } else {
        surrounding.push(Some(grid[x_u+1][y_u-1]));
    }

    // W
    if x - 1 < 0 {
        surrounding.push(None);
    } else {
        surrounding.push(Some(grid[x_u-1][y_u]));
    }

    // E
    if x + 1 > max_x {
        surrounding.push(None);
    } else {
        surrounding.push(Some(grid[x_u+1][y_u]));
    }

    // SW
    if x - 1 < 0 || y + 1 > max_y {
        surrounding.push(None);
    } else {
        surrounding.push(Some(grid[x_u-1][y_u+1]));
    }

    // S
    if y + 1 > max_y {
        surrounding.push(None);
    } else {
        surrounding.push(Some(grid[x_u][y_u+1]));
    }

    // SE
    if x + 1 > max_x || y + 1 > max_y {
        surrounding.push(None);
    } else {
        surrounding.push(Some(grid[x_u+1][y_u+1]));
    }

    return surrounding;
}

fn count_adjacent_rolls(surrounding: Vec<Option<&str>>) -> u32 {
    let mut total: u32 = 0;

    for position in surrounding {
        if position.is_none() || position.unwrap() != "@" {
            continue;
        }

        total += 1;
    }

    return total;
}

pub fn one() {
    let lines: Vec<&str> = get_input()
        .split("\n")
        .filter(|s| *s != "")
        .collect();
    let mut grid: Vec<Vec<&str>> = Vec::new();

    for _ in 0..lines[0].len() as u32 {
        grid.push(Vec::new());
    }

    for line in &lines {
        for x in 0..line.len() as u32 {
            let x_u = x as usize;
            grid[x_u].push(line.get(x_u..x_u + 1).unwrap());
        }
    }

    let mut y = 0;
    while y < 138 {
        for x in 0..lines[0].len() as u32 {
            assert_eq!(
                grid[x as usize][y as usize],
                lines[y as usize].get(x as usize..x as usize + 1).unwrap()
            );
        }
        y += 1;
    }
    
    let mut total: u32 = 0;
    for x in 0..138 {
        for y in 0..138 {
            if grid[x][y] != "@" {
                continue;
            }

            let surrounding: Vec<Option<&str>> = get_surrounding(&grid, x as i32, y as i32);
            let count = count_adjacent_rolls(surrounding);

            if count < 4 {
                total += 1;
            }
        }
    }

    println!("{total}");
}
