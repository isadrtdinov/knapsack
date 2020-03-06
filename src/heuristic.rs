use crate::Item;

mod dynamic {
    use crate::Item;
    use std::cmp::max;

    fn dynamic_table(capacity: u32, items: &Vec::<Item>,
        left: usize, right: usize,
    ) -> (u32, u32, Vec::<u32>, Vec::<u32>)
    {
        let mut old_table = vec![0; capacity as usize + 1];
        let mut left_table = Vec::new();

        let middle = (left + right) / 2;
        for i in left..right {
            if i == middle {
                left_table = old_table.clone();
            }

            let item = &items[i];

            if item.weight > capacity {
                continue;
            }

            let mut table = vec![0; capacity as usize + 1];

            for j in 0..item.weight as usize {
                table[j] = old_table[j];
            }

            table[item.weight as usize] = max(old_table[item.weight as usize], item.value);

            for j in item.weight as usize + 1..capacity as usize + 1 {
                if table[j - item.weight as usize] != 0 {
                    table[j] = max(old_table[j], old_table[j - item.weight as usize] + item.value);
                } else {
                    table[j] = old_table[j];
                }
            }

            old_table = table;
        }

        let (weight, value) = old_table.iter().enumerate()
            .max_by_key(|&(_, item)| item).unwrap();
        (*value, weight as u32, old_table, left_table)
    }

    fn restore_items(items: &Vec::<Item>,
        value: u32, weight: u32,
        left: usize, right: usize,
        left_table: Option::<Vec::<u32>>,
        taken: &mut Vec::<usize>)
    {
        if weight == 0 {
            return;
        }

        if right - left == 1 {
            taken.push(items[left].index);
            return;
        }

        let middle = (left + right) / 2;

        let (left_table, child_left_table) = match left_table {
            Some(table) => (table, None),
            None => {
                let (_, _, left_table, child_left_table) = dynamic_table(
                    weight, items,
                    left, middle,
                );
                (left_table, Some(child_left_table))
            }
        };

        let (_, _, right_table, child_right_table) = dynamic_table(
            weight, items,
            middle, right,
        );

        let (left_weight, left_value) = {
            let mut left_weight = 0;
            let mut left_value = 0;
            for i in 0..weight as usize + 1 {
                if left_table[i] + right_table[weight as usize - i] == value {
                    if left_table[i] == 0 && i > 0 {
                        continue;
                    }

                    if right_table[weight as usize - i] == 0 && i < weight as usize {
                        continue;
                    }

                    left_weight = i as u32;
                    left_value = left_table[i];
                    break;
                }
            }

            (left_weight, left_value)
        };

        drop(left_table);
        drop(right_table);

        restore_items(items,
            left_value, left_weight,
            left, middle,
            child_left_table,
            taken,
        );

        restore_items(items,
            value - left_value, weight - left_weight,
            middle, right,
            Some(child_right_table),
            taken,
        );
    }

    pub fn dynamic(capacity: u32, items: &Vec::<Item>) -> (u32, Vec::<usize>) {
        let (value, weight, left_table) = {
            let (value, weight, _, left_table) = dynamic_table(
                capacity, items,
                0, items.len(),
            );
            (value, weight, left_table)
        };

        let mut taken = Vec::new();
        restore_items(items,
            value, weight,
            0, items.len(),
            Some(left_table),
            &mut taken,
        );

        (value, taken)
    }
}

pub fn heuristic(capacity: u32, items: &Vec::<Item>,
    capacity_part: f64, window_size: usize,
) -> (u32, Vec::<usize>) {
    let capacity_border = ((capacity as f64) * capacity_part) as u32;
    let mut greedy_weight = 0;
    let mut greedy_value = 0;
    let mut last_taken = 0;

    let mut greedy_taken = Vec::new();
    let mut filtered_items = Vec::new();

    for (i, item) in items.iter().enumerate() {
        if last_taken == 0 {
            if greedy_weight + item.weight > capacity_border {
                last_taken = i;
                filtered_items.push(item.clone());
                continue;
            }

            greedy_weight += item.weight;
            greedy_value += item.value;
            greedy_taken.push(item.index);

        } else {

            filtered_items.push(item.clone());

            if i == last_taken + window_size - 1 {
                break;
            }
        }
    }

    let (value, taken) = dynamic::dynamic(capacity - greedy_weight, &filtered_items);

    let taken = [&greedy_taken[..], &taken[..]].concat().to_vec();

    (greedy_value + value, taken)
}
