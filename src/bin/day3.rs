fn main() {
    let regex = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;

    for line in advent_of_code_2024::LineIter::new() {
        for m in regex.captures_iter(&line) {
            let a: i64 = m[1].parse().unwrap();
            let b: i64 = m[2].parse().unwrap();

            let r = a * b;
            eprintln!("{a} x {b} = {r}");

            sum += r;
        }
    }

    eprintln!("total: {sum}");
}
