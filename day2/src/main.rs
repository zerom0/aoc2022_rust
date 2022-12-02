use std::io;

fn main() {
    let rounds: usize = io::stdin()
        .lines()
        .map(|round| match round.unwrap().split_once(" ") {
            Some((l, r)) => evaluate(l, r),
            None => panic!(),
        })
        .sum();
    println!("{:?}", rounds);
}

fn evaluate(opponent: &str, you: &str) -> usize {
    let opponent_index = match opponent {
        "A" => 0, // rock
        "B" => 1, // paper
        "C" => 2, // scissors
        _ => panic!(),
    };

    let loose_agains = vec![2, 0, 1];
    let draw_against = vec![0, 1, 2];
    let win_against = vec![1, 2, 0];

    let your_index = match you {
        "X" => loose_agains[opponent_index],
        "Y" => draw_against[opponent_index],
        "Z" => win_against[opponent_index],
        _ => panic!(),
    };

    let matrix = vec![vec![3, 6, 0], vec![0, 3, 6], vec![6, 0, 3]];

    your_index + 1 + matrix[opponent_index][your_index]
}
