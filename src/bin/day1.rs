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

    let mut sum = 0;
    for k in 0..list1.len() {
        let delta = (list1[k] - list2[k]).abs();

        sum += delta;
    }

    eprintln!("sum: {sum}");
}
