use std::io;

fn main() {
    let rounds: u32 = io::stdin()
        .lines()
        .map(|line| {
            let items = line.unwrap();
            let item_count = items.len();
            let (l, r) = items.split_at(item_count / 2);
            common(l, r)
        })
        .sum();
    println!("{:?}", rounds);
}

fn common(l: &str, r: &str) -> u32 {
    for c in l.chars() {
        if r.contains(c) {
            if c >= 'a' {
                return c as u32 - 'a' as u32 + 1;
            } else {
                return c as u32 - 'A' as u32 + 27;
            }
        }
    }
    0
}
