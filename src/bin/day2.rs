fn main() {
    let mut safe_cnt = 0;

    'next_reading: for line in advent_of_code_2024::LineIter::new() {
        let levels: Vec<i32> = line.split(' ').map(|n| n.parse().unwrap()).collect();

        if levels.len() < 2 {
            safe_cnt += 1;
            continue;
        }

        let increasing = levels[1] > levels[0];
        for i in 1..levels.len() {
            let a = levels[i - 1];
            let b = levels[i];

            if !increasing ^ (a > b) {
                continue 'next_reading;
            }

            let delta = (a - b).abs();

            if !(1..=3).contains(&delta) {
                continue 'next_reading;
            }
        }

        safe_cnt += 1;
    }

    eprintln!("Safe count: {safe_cnt}");
}
