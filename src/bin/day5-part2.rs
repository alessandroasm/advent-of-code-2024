use std::collections::HashMap;

fn main() {
    let lines = advent_of_code_2024::LineIter::new();
    let mut map = HashMap::new();

    for line in lines {
        if line.is_empty() {
            break;
        }

        let (a, b) = line.split_once('|').unwrap();

        let a: u32 = a.parse().unwrap();
        let b: u32 = b.parse().unwrap();

        map.entry(b)
            .and_modify(|v: &mut Vec<u32>| v.push(a))
            .or_insert_with(|| vec![a]);
    }

    let lines = advent_of_code_2024::LineIter::new();
    let mut sum = 0;

    for line in lines {
        let mut values: Vec<u32> = line.split(',').map(|n| n.parse().unwrap()).collect();
        let mut was_invalid = false;

        while let Some((idx0, idx1)) = is_invalid(&map, &values) {
            was_invalid = true;
            values.swap(idx0, idx1);
        }

        if was_invalid {
            eprintln!("line matches: {line}");

            let idx = values.len() / 2;
            sum += values[idx];
        }
    }

    eprintln!("Sum: {sum}");
}

fn is_invalid(map: &HashMap<u32, Vec<u32>>, values: &Vec<u32>) -> Option<(usize, usize)> {
    let mut set: HashMap<&u32, usize> = HashMap::new();

    for (idx, v) in values.iter().enumerate() {
        if let Some(idx0) = set.get(v) {
            return Some((*idx0, idx));
        }

        if let Some(rules) = map.get(v) {
            rules.iter().for_each(|v| {
                set.entry(v).or_insert(idx);
            });
        }
    }

    None
}
