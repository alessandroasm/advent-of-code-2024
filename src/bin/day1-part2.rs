use std::collections::HashMap;

fn main() {
    let re = regex::Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let mut list1 = vec![];
    let mut list2 = vec![];

    for line in advent_of_code_2024::LineIter::new() {
        eprintln!("ln: {line}");

        let captures = re.captures(&line).unwrap();
        let n1: i32 = captures[1].parse().unwrap();
        let n2: i32 = captures[2].parse().unwrap();

        list1.push(n1);
        list2.push(n2);
    }

    list1.sort();
    list2.sort();

    let mut list2_counters = HashMap::new();
    list2.into_iter().for_each(|n| {
        list2_counters.entry(n).and_modify(|c| *c += 1).or_insert(1);
    });

    let mut sum = 0;
    list1
        .into_iter()
        .for_each(|n| sum += n * list2_counters.get(&n).unwrap_or(&0i32));

    eprintln!("sum: {sum}");
}
