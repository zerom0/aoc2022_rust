use std::io;

fn main() {
    let full_oversaps = io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_once(',')
                .map(|(first, second)| {
                    let first_range = if let Some((from, to)) = first.split_once('-') {
                        (from.parse::<u32>().unwrap(), to.parse::<u32>().unwrap())
                    } else {
                        panic!()
                    };

                    let second_range = if let Some((from, to)) = second.split_once('-') {
                        (from.parse::<u32>().unwrap(), to.parse::<u32>().unwrap())
                    } else {
                        panic!()
                    };

                    if contains(first_range, second_range) || contains(second_range, first_range) {
                        1
                    } else {
                        0
                    }
                })
                .unwrap()
        })
        .sum::<i32>();
    println!("{:?}", full_oversaps);
}

fn contains(left: (u32, u32), right: (u32, u32)) -> bool {
    left.0 <= right.0 && left.1 >= right.1
}
