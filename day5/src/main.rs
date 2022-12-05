use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines().map(|line| line.unwrap()).collect();
    let mut it = lines.split(|line| line.is_empty());

    // Convert first half of input into working_stacks
    let mut stacks = it.next().unwrap().to_vec();
    stacks.reverse();
    let mut stack_iter = stacks.iter();
    let commands = it.next().unwrap();
    let columns = stack_iter
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    // println!("{:?}", columns);

    let mut working_stacks = Vec::<Vec<char>>::new();
    for _ in 0..columns {
        working_stacks.push(Vec::<char>::new());
    }
    for level in stack_iter {
        for i in 0..columns {
            if level.len() > i * 4 + 1 {
                let item = level.as_bytes()[i * 4 + 1] as char;
                if item != ' ' {
                    working_stacks[i].push(level.as_bytes()[i * 4 + 1] as char);
                }
            }
        }
    }
    // println!("{:?}", working_stacks);

    for line in commands {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let count = parts[1].parse::<usize>().unwrap();
        let from = parts[3].parse::<usize>().unwrap();
        let to = parts[5].parse::<usize>().unwrap();
        let mut items = Vec::new();
        for _ in 0..count {
            items.push(working_stacks[from - 1].pop().unwrap());
        }
        items.reverse();
        // println!("{:?}", items);
        working_stacks[to - 1].append(&mut items);
        // println!("{} * {} -> {}", count, from, to);
        // println!("{:?}", working_stacks);
    }
    for stack in working_stacks {
        print!("{}", stack.last().unwrap());
    }
}
