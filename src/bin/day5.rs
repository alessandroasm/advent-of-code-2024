use std::collections::{HashMap, HashSet};

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

    'next_line: for line in lines {
        let values: Vec<u32> = line.split(',').map(|n| n.parse().unwrap()).collect();
        let mut set = HashSet::new();

        for v in values.iter() {
            if set.contains(v) {
                continue 'next_line;
            }

            if let Some(rules) = map.get(v) {
                rules.iter().for_each(|v| {
                    set.insert(v);
                });
            }
        }

        eprintln!("line matches: {line}");

        let idx = values.len() / 2;
        sum += values[idx];
    }

    eprintln!("Sum: {sum}");
}
