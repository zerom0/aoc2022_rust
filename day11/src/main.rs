use num_bigint::BigUint;
use std::rc::Rc;

fn main() {
    let mut monkeys = std::io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .map(|monkey| {
            let mut v = monkey.to_vec();
            v.reverse();
            get_line(&mut v);
            Monkey {
                items: get_line(&mut v)
                    .split_once(": ")
                    .unwrap()
                    .1
                    .split(", ")
                    .map(|item| item.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
                operation: {
                    match get_line(&mut v)
                        .split_once("old ")
                        .unwrap()
                        .1
                        .split_once(' ')
                    {
                        Some(("*", "old")) => Rc::new(|x: usize| x.clone() * x.clone()),
                        Some(("*", a)) => {
                            let factor = a.parse::<usize>().unwrap();
                            Rc::new(move |x: usize| x * factor)
                        } // how to get the value of v into the lambda?
                        Some(("+", a)) => {
                            let factor = a.parse::<usize>().unwrap();
                            Rc::new(move |x: usize| x + factor)
                        }
                        _ => panic!(),
                    }
                },
                test_prime: value_after_pattern(&mut v, "by "),
                on_true: value_after_pattern(&mut v, "monkey "),
                on_false: value_after_pattern(&mut v, "monkey "),
            }
        })
        .collect::<Vec<_>>();

    let prime_product = calc_prime_product(&monkeys);

    let mut inspection = vec![0; monkeys.len()];
    for round in 0..10000 {
        // while let Some(monkey) = monkey_items.pop() {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            monkeys[i].items.clear();
            for item in monkey.items {
                inspection[i] += 1;
                let worry_level = (monkey.operation)(item);
                let target = if &worry_level % monkey.test_prime == 0 {
                    monkey.on_true
                } else {
                    monkey.on_false
                };
                // println!(
                //     "{} -> {} -> monkey {}",
                //     item, worry_level, target
                // );
                monkeys[target].items.push(worry_level % prime_product);
            }
        }
        if (round + 1) % 1000 == 0 {
            println!("{}:", round + 1);
            for x in &inspection {
                println!("  {}", x);
            }
        }
    }
    inspection.sort();
    println!(
        "{}",
        BigUint::from(inspection.pop().unwrap() as usize)
            * BigUint::from(inspection.pop().unwrap() as usize)
    );
}

fn get_line(v: &mut Vec<String>) -> String {
    v.pop().unwrap()
}

fn value_after_pattern(v: &mut Vec<String>, pattern: &str) -> usize {
    get_line(v)
        .split_once(pattern)
        .unwrap()
        .1
        .parse::<usize>()
        .unwrap()
}

fn calc_prime_product(monkeys: &Vec<Monkey>) -> usize {
    monkeys
        .iter()
        .fold(1, |pre, monkey| pre * monkey.test_prime)
}

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Rc<dyn Fn(usize) -> usize>,
    test_prime: usize,
    on_true: usize,
    on_false: usize,
}
