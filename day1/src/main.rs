use std::io;

fn main() {
    let bag = io::stdin()
        .lines()
        .fold(Vec::<Vec<u32>>::new(), |mut elves, line| {
            let text = line.unwrap();
            if elves.is_empty() {
                elves.push(Vec::<u32>::new());
            }
            if text.is_empty() {
                elves.push(Vec::<u32>::new());
            } else {
                elves.last_mut().unwrap().push(text.parse::<u32>().unwrap());
            }
            elves
        });

    let mut cals_in_bag = bag
        .into_iter()
        .map(|bag| bag.into_iter().sum::<u32>())
        .collect::<Vec<u32>>();

    // let mut max_cals = 0_u32;
    // let mut max_cals_index = 0;
    // for (index, cals) in cals_in_bag.into_iter().enumerate() {
    //     if cals > max_cals {
    //         max_cals = cals;
    //         max_cals_index = index;
    //     }
    // }
    cals_in_bag.sort();
    let sum_of_three_max_cals = cals_in_bag.into_iter().rev().take(3).sum::<u32>();
    println!("{:?}", sum_of_three_max_cals);
}
