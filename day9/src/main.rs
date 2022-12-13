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
            if head_pos.y > self.y {
                self.y += 1;
            } else if head_pos.y < self.y {
                self.y -= 1;
            }
        }
        if head_pos.x - self.x == -2 {
            self.x -= 1;
            if head_pos.y > self.y {
                self.y += 1;
            } else if head_pos.y < self.y {
                self.y -= 1;
            }
        }
        if head_pos.y - self.y == 2 {
            self.y += 1;
            if head_pos.x > self.x {
                self.x += 1;
            } else if head_pos.x < self.x {
                self.x -= 1;
            }
        }
        if head_pos.y - self.y == -2 {
            self.y -= 1;
            if head_pos.x > self.x {
                self.x += 1;
            } else if head_pos.x < self.x {
                self.x -= 1;
            }
        }
    }
}

fn show(knots: &Vec<Position>) {
    (-5..16).rev().for_each(|y| {
        (-11..15).for_each(|x| {
            let mut marker = String::from(".");
            if x == 0 && y == 0 {
                marker = "s".to_string();
            }
            for i in 0..knots.len() {
                if knots[i].x == x && knots[i].y == y {
                    marker = (knots.len() - i - 1).to_string();
                }
            }
            print!("{}", marker);
        });
        println!();
    })
}

fn main() {
    let lines = std::io::stdin().lines().map(|line| line.unwrap());
    let mut knots = vec![Position { x: 0, y: 0 }];
    (0..9).for_each(|_| knots.push(Position { x: 0, y: 0 }));
    let mut tail_positions = vec![Position { x: 0, y: 0 }];
    for line in lines {
        let (direction, count) = line.split_once(' ').unwrap();
        // repeat steps 'count' times
        for _ in 0..count.to_string().parse::<usize>().unwrap() {
            knots.last_mut().unwrap().move_one(direction);
            for i in (0..(knots.len() - 1)).rev() {
                // follow next knot
                let last = knots[i + 1].clone();
                knots[i].follow(&last);
            }
            tail_positions.push(knots[0].clone());
        }
        println!("{} {}", count, direction);
        show(&knots);
    }
    tail_positions.sort();
    tail_positions.dedup();
    println!("{:?}", tail_positions.len());
}
