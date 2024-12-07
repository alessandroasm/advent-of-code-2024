fn check_safety(levels: &[i32]) -> Option<usize> {
    if levels.len() < 2 {
        return None;
    }

    let increasing = levels[1] > levels[0];
    for i in 1..levels.len() {
        let a = levels[i - 1];
        let b = levels[i];

        if !increasing ^ (a > b) {
            return Some(i);
        }

        let delta = (a - b).abs();

        if !(1..=3).contains(&delta) {
            return Some(i);
        }
    }

    None
}

fn check_safety_masked(levels: &[i32], masked_idx: usize) -> Option<usize> {
    let p0 = levels[0..masked_idx].iter();
    let p1 = levels[masked_idx + 1..].iter();

    let masked_levels: Vec<_> = p0.chain(p1).map(|n| *n).collect();

    let r = check_safety(&masked_levels);
    eprintln!("{levels:?} -> {masked_levels:?} ({masked_idx}): {r:?}");
    r
}

fn main() {
    let mut safe_cnt = 0;

    for line in advent_of_code_2024::LineIter::new() {
        let levels: Vec<i32> = line.split(' ').map(|n| n.parse().unwrap()).collect();

        let unsafe_level_idx = check_safety(&levels).and_then(|n| {
            println!("n: {n}");
            let r =
                check_safety_masked(&levels, n).and_then(|_| check_safety_masked(&levels, n - 1));
            if let Some(2) = r {
                check_safety_masked(&levels, 0)
            } else {
                r
            }
        });

        if unsafe_level_idx.is_none() {
            safe_cnt += 1;
        }

        // if check_safety(&levels).is_none() {
        //     safe_cnt += 1;
        // } else {
        //     for k in 0..levels.len() {
        //         if check_safety_masked(&levels, k).is_none() {
        //             println!("{levels:?}, {k}");
        //             safe_cnt += 1;
        //             break;
        //         }
        //     }
        // }
    }

    eprintln!("Safe count: {safe_cnt}");
}
