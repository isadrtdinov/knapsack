mod ibarra_kim;
mod heuristic;

use std::io::{self, Read};

#[derive(Clone)]
pub struct Item {
    index: usize,
    value: u32,
    weight: u32,
}

fn process_input(input_data: &String) -> (u32, Vec::<Item>) {
    let lines: Vec<_> = input_data.split("\n").collect();

    let first_line: Vec<_> = lines[0].split(" ").collect();
    let capacity = first_line[1].parse::<u32>().expect("capacity");

    let mut items = Vec::new();

    for i in 1..lines.len() - 1 {
        let parts: Vec<_> = lines[i].split(" ").collect();
        let value = parts[0].parse::<u32>().expect("value");
        let weight = parts[1].parse::<u32>().expect("weight");

        if weight <= capacity {
            items.push(
                Item {
                    index: i,
                    value,
                    weight,
                }
            );
        }
    }

    items.sort_by(
        |first, second| {
            let first_cost = -(first.value as f64) / (first.weight as f64);
            let second_cost = -(second.value as f64) / (second.weight as f64);
            first_cost.partial_cmp(&second_cost).unwrap()
        }
    );

    (capacity, items)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut input_data = String::new();
    handle.read_to_string(&mut input_data)?;

    let (capacity, items) = process_input(&input_data);

    let eps = if items.len() <= 50000 {
        0.0195
    } else {
        0.04
    };

    let (mut value, mut taken) = heuristic::heuristic(capacity, &items, 0.5, 1200);
    let (ik_value, ik_taken) = ibarra_kim::ibarra_kim(capacity, &items, eps);

    if ik_value > value {
        value = ik_value;
        taken =  ik_taken;
    }

    let indices: Vec<_> = taken.into_iter()
        .map(|x| x.to_string()).collect();
    println!("{}\n{}", value, &indices[..].join(" "));

    Ok(())
}
