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
    let mut visible_trees = 0;
    (0..rows).for_each(|row| {
        (0..cols).for_each(|col| {
            if row == 0 || row == (rows - 1) || col == 0 || col == (cols - 1) {
                visible_trees += 1;
            } else if left_lower(row, col, &input)
                || right_lower(row, col, &input)
                || top_lower(row, col, &input)
                || bottom_lower(row, col, &input)
            {
                visible_trees += 1;
            }
        })
    });
    println!("Visible trees = {}", visible_trees);
}

fn left_lower(row: usize, col: usize, input: &Vec<Vec<u32>>) -> bool {
    for index in 0..col {
        if input[row][index] >= input[row][col] {
            return false;
        }
    }
    true
}

fn right_lower(row: usize, col: usize, input: &Vec<Vec<u32>>) -> bool {
    for index in (col + 1)..input.first().unwrap().len() {
        if input[row][index] >= input[row][col] {
            return false;
        }
    }
    true
}

fn top_lower(row: usize, col: usize, input: &Vec<Vec<u32>>) -> bool {
    for index in 0..row {
        if input[index][col] >= input[row][col] {
            return false;
        }
    }
    true
}

fn bottom_lower(row: usize, col: usize, input: &Vec<Vec<u32>>) -> bool {
    for index in (row + 1)..input.len() {
        if input[index][col] >= input[row][col] {
            return false;
        }
    }
    true
}
