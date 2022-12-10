fn main() {
    let input = std::io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();
    let rows = input.len();
    let cols = input.first().unwrap().len();
    let mut max_visibility = 0;
    (0..rows).for_each(|row| {
        (0..cols).for_each(|col| {
            print!("{}x{}: ", row, col);
            let visibility = left_dist(row, col, &input)
                * right_dist(row, col, &input)
                * top_dist(row, col, &input)
                * bottom_dist(row, col, &input);
            if visibility > max_visibility {
                max_visibility = visibility;
            }
            println!();
        })
    });
    println!("Visible trees = {}", max_visibility);
}

fn left_dist(row: usize, col: usize, input: &Vec<Vec<u32>>) -> u32 {
    let mut dist = 0;
    for index in (0..col).rev() {
        dist += 1;
        if input[row][index] >= input[row][col] {
            break;
        }
    }
    print!("{} ", dist);
    dist
}

fn right_dist(row: usize, col: usize, input: &Vec<Vec<u32>>) -> u32 {
    let mut dist = 0;
    for index in (col + 1)..input.first().unwrap().len() {
        dist += 1;
        if input[row][index] >= input[row][col] {
            break;
        }
    }
    print!("{} ", dist);
    dist
}

fn top_dist(row: usize, col: usize, input: &Vec<Vec<u32>>) -> u32 {
    let mut dist = 0;
    for index in (0..row).rev() {
        dist += 1;
        if input[index][col] >= input[row][col] {
            break;
        }
    }
    print!("{} ", dist);
    dist
}

fn bottom_dist(row: usize, col: usize, input: &Vec<Vec<u32>>) -> u32 {
    let mut dist = 0;
    for index in (row + 1)..input.len() {
        dist += 1;
        if input[index][col] >= input[row][col] {
            break;
        }
    }
    print!("{} ", dist);
    dist
}
