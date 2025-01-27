use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin().lock();
    let mut sum = 0;

    for line in stdin.lines() {
        let line = line.unwrap();
        let (res_str, numbers_str) = line.split_once(": ").unwrap();

        let result = res_str.parse::<i64>().unwrap();

        let numbers: Vec<_> = numbers_str
            .split(' ')
            .map(|n| n.parse::<i64>().unwrap())
            .collect();

        //eprintln!("res: {result}, n: {numbers:?}");

        if test_result(result, numbers[0], 0, &numbers) {
            sum += result;
        }
    }

    eprintln!("sum: {sum}");
}

fn test_result(result: i64, x: i64, k: usize, numbers: &Vec<i64>) -> bool {
    if k == numbers.len() - 1 {
        return result == x;
    }

    let n = numbers[k + 1];
    return test_result(result, x + n, k + 1, numbers)
        || test_result(result, x * n, k + 1, numbers);
}
