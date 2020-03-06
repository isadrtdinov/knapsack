use crate::Item;

mod dynamic {
    use crate::Item;
    use std::cmp::min;

    pub fn greedy(capacity: u32, sorted_items: &Vec::<Item>) -> (u32, Vec::<usize>) {
        if sorted_items.len() == 0 {
            return (0, Vec::new());
        }

        let mut value: u32 = 0;
        let mut weight: u32 = 0;
        let mut taken = Vec::new();

        let mut last = 0;
        for (i, item) in sorted_items.iter().enumerate() {
            if weight + item.weight > capacity {
                last = i;
                break;
            }

            value += item.value;
            weight += item.weight;
            taken.push(item.index);
        }

        if value < sorted_items[last].value && sorted_items[last].weight <= capacity {
            return (sorted_items[last].value, vec![sorted_items[last].index]);
        }

        (value, taken)
    }

    pub fn dynamic_table(capacity: u32, max_value: u32,
        items: &Vec::<Item>) -> Vec::<Vec::<u32>> {
        let mut table = Vec::new();

        if items.len() == 0 {
            return Vec::new();
        }

        table.push(vec![capacity + 1; max_value as usize + 1]);
        table[0][items[0].value as usize] = items[0].weight;

        for i in 1..items.len() {
            table.push(vec![capacity + 1; max_value as usize + 1]);

            let item = &items[i];
            for j in 0..item.value as usize {
                table[i][j] = table[i - 1][j];
            }

            table[i][item.value as usize] = min(
                table[i - 1][item.value as usize],
                item.weight,
            );

            for j in item.value as usize + 1..max_value as usize + 1 {
                table[i][j] = min(
                    table[i - 1][j],
                    table[i - 1][j - item.value as usize] + item.weight,
                );
            }
        }

        table
    }

    pub fn restore_items(value: u32, items: &Vec::<Item>,
        table: &Vec::<Vec::<u32>>
    ) -> Vec::<usize> {
        let mut taken = Vec::new();
        let mut j = value as usize;

        for i in (1..items.len()).rev() {
            if table[i][j] != table[i - 1][j] {
                j -= items[i].value as usize;
                taken.push(items[i].index);
            }
        }

        if j > 0 {
            taken.push(items[0].index);
        }

        taken
    }
}

pub fn ibarra_kim(capacity: u32, items: &Vec::<Item>, eps: f64) -> (u32, Vec::<usize>) {
    let (greedy_value, greedy_taken) = dynamic::greedy(capacity, items);

    let factor = 1.0 / (eps * eps * 2.0 * greedy_value as f64);
    let border = 1.0 / eps;

    let expensive_items: Vec<_> = items.iter().filter(
            |item| (item.value as f64) * factor > border,
        ).enumerate().map(
            |(i, item)| {
                Item {
                    index: i,
                    value: (item.value as f64 * factor) as u32,
                    weight: item.weight,
                }
            }
        ).collect();

    let cheap_items: Vec<_> = items.iter().filter(
            |item| (item.value as f64) * factor <= border,
        ).enumerate().map(
            |(i, item)| {
                Item {
                    index: i,
                    value: item.value,
                    weight: item.weight,
                }
            }
        ).collect();

    if expensive_items.len() == 0 {
        return (greedy_value, greedy_taken);
    }

    let table = dynamic::dynamic_table(
        capacity, (1.0 / eps / eps) as u32,
        &expensive_items,
    );

    let mut max_value = 0;
    let mut best_value = 0;
    let mut cheap_taken = Vec::new();

    for (value, &weight) in table[table.len() - 1].iter().enumerate() {
        if weight > capacity {
            continue;
        }

        let (cheap_value, taken) = dynamic::greedy(capacity - weight, &cheap_items);

        let new_value = ((value as f64) / factor) as u32 + cheap_value;
        if new_value > max_value {
            max_value = new_value;
            best_value = value as u32;
            cheap_taken = taken;
        }
    }

    let expensive_taken = dynamic::restore_items(
        best_value,
        &expensive_items,
        &table,
    );

    let taken_indices = [&cheap_taken[..], &expensive_taken[..]].concat();

    let value = taken_indices.iter().map(
            |&i| items[i].value,
    ).sum();

    if value < greedy_value {
        return (greedy_value, greedy_taken);
    }

    let taken: Vec<_> = taken_indices.iter().map(
        |&i| items[i].index,
    ).collect();

    (value, taken)
}
