use std::io;

fn main() {
    let mut lines = io::stdin().lines().map(|line| line.unwrap()).into_iter();
    let mut priorities = Vec::<u32>::new();
    loop {
        let first = if let Some(n) = lines.next() { n } else { break };
        let second = lines.next().unwrap();
        let third = lines.next().unwrap();
        let common_item = common(&common(&first, &second), &third)
            .chars()
            .next()
            .unwrap();
        priorities.push(priority(common_item));
    }
    println!("{:?}", priorities.iter().sum::<u32>());
}

fn common(l: &str, r: &str) -> String {
    let mut common_items = String::new();
    for c in l.chars() {
        if r.contains(c) {
            common_items.push(c);
        }
    }
    common_items
}

fn priority(c: char) -> u32 {
    if c >= 'a' {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}
