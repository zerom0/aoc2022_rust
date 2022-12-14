fn main() {
    let lines = std::io::stdin().lines().map(|line| line.unwrap());
    let mut cycles = vec![1_i32];
    let mut state = 1_i32;
    for line in lines {
        if line == "noop" {
            cycles.push(state);
        } else {
            let (_, amount) = line.split_once(' ').unwrap();
            cycles.push(state);
            cycles.push(state);
            state += amount.parse::<i32>().unwrap();
        }
    }
    let mut sum = 0;
    for i in [20, 60, 100, 140, 180, 220] {
        println!("{} * {} = {}", i, cycles[i], i as i32 * cycles[i]);
        sum += i as i32 * cycles[i];
    }
    println!("Sum = {}", sum);
}
