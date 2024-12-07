fn main() {
    let regex = regex::Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do(n't)?\(\))").unwrap();
    let mut sum = 0;

    let mut enabled = true;

    for line in advent_of_code_2024::LineIter::new() {
        for m in regex.captures_iter(&line) {
            let op = &m[0];
            if op.starts_with("don't") {
                enabled = false;
            } else if op.starts_with("do") {
                enabled = true;
            } else {
                if !enabled {
                    continue;
                }

                eprintln!("m1: {}, m2: {}", &m[2], &m[3]);

                let a: i64 = m[2].parse().unwrap();
                let b: i64 = m[3].parse().unwrap();

                let r = a * b;
                eprintln!("{a} x {b} = {r}");

                sum += r;
            }
        }
    }

    eprintln!("total: {sum}");
}
