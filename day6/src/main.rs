use std::io;

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let line = lines.first().unwrap();
    let pattern_let = 14;
    for i in 0..(line.len() - pattern_let) {
        let slice = &line[i..i + pattern_let];
        if all_unique(slice) {
            println!("{:?}", i + pattern_let);
            break;
        }
    }
}

fn all_unique(segment: &str) -> bool {
    //    print!("{:?} -> ", segment);
    let mut bytes = segment.to_string().into_bytes();
    bytes.sort();
    //    println!("{:?}", bytes);
    for i in 0..(bytes.len() - 1) {
        if bytes[i] == bytes[i + 1] {
            return false;
        }
    }
    true
}
