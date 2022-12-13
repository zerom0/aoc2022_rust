#[derive(Debug, Clone, Eq, PartialEq, Ord)]
struct Position {
    x: isize,
    y: isize,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.x == other.x {
            Some(self.y.cmp(&other.y))
        } else {
            Some(self.x.cmp(&other.x))
        }
    }
}

impl Position {
    fn move_one(&mut self, dir: &str) {
        (self.x, self.y) = match dir {
            "R" => (self.x + 1, self.y),
            "U" => (self.x, self.y + 1),
            "L" => (self.x - 1, self.y),
            "D" => (self.x, self.y - 1),
            _ => panic!(),
        }
    }

    fn follow(&mut self, head_pos: &Position) {
        if head_pos.x - self.x == 2 {
            self.x += 1;
            if head_pos.y != self.y {
                self.y = head_pos.y;
            }
        }
        if head_pos.x - self.x == -2 {
            self.x -= 1;
            if head_pos.y != self.y {
                self.y = head_pos.y;
            }
        }
        if head_pos.y - self.y == 2 {
            self.y += 1;
            if head_pos.x != self.x {
                self.x = head_pos.x;
            }
        }
        if head_pos.y - self.y == -2 {
            self.y -= 1;
            if head_pos.x != self.x {
                self.x = head_pos.x;
            }
        }
    }
}

fn show(head: &Position, tail: &Position) {
    (0..5).rev().for_each(|y| {
        (0..5).for_each(|x| {
            if head.x == x && head.y == y {
                print!("H");
            } else if tail.x == x && tail.y == y {
                print!("T");
            } else {
                print!(".");
            }
        });
        println!();
    })
}

fn main() {
    let lines = std::io::stdin().lines().map(|line| line.unwrap());
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };
    let mut tail_positions = vec![tail.clone()];
    for line in lines {
        let (direction, count) = line.split_once(' ').unwrap();
        for _ in 0..count.to_string().parse::<usize>().unwrap() {
            head.move_one(direction);
            tail.follow(&head);
            tail_positions.push(tail.clone());
            println!("{} Head {:?} Tail {:?}", direction, head, tail);
            show(&head, &tail)
        }
    }
    tail_positions.sort();
    tail_positions.dedup();
    println!("{:?}", tail_positions.len());
}
