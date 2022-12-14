fn main() {
    let lines = std::io::stdin().lines().map(|line| line.unwrap());
//    let mut cycles = vec![1_i32];
    let mut current_cycle = 1_i32;
    let mut state = 1_i32;
    for line in lines {
        if line == "noop" {
            next_cycle(&mut current_cycle, &state);
        } else {
            let (_, amount) = line.split_once(' ').unwrap();
            next_cycle(&mut current_cycle, &state);
            next_cycle(&mut current_cycle, &state);
            state += amount.parse::<i32>().unwrap();
        }
    }
}

fn next_cycle(cycle: &mut i32, state: &i32) {
    if *cycle >= *state && *cycle <= (state + 2) {
        print!("#");
    } else {
        print!(".");
    }
    if *cycle % 40 == 0 {
        println!();
        *cycle = 0;
    }
    *cycle += 1;
}