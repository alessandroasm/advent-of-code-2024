fn main() {
    let map: Vec<Vec<_>> = advent_of_code_2024::LineIter::new()
        .map(|ln| ln.chars().collect())
        .collect();

    let mut cnt = 0;

    let len_x = map[0].len();
    let len_y = map.len();

    map.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, ch)| {
            if *ch == 'A' {
                let (Some(prev_x), Some(next_x), Some(prev_y), Some(next_y)) = (
                    next_pos(x, len_x, Direction::Back),
                    next_pos(x, len_x, Direction::Forward),
                    next_pos(y, len_y, Direction::Back),
                    next_pos(y, len_y, Direction::Forward),
                ) else {
                    return;
                };

                let d1 = (&map[prev_y][prev_x], &map[next_y][next_x]);
                let d2 = (&map[next_y][prev_x], &map[prev_y][next_x]);

                if is_xmas(d1) && is_xmas(d2) {
                    cnt += 1;
                }
            }
        });
    });

    eprintln!("Count: {cnt}");
}

fn is_xmas(d: (&char, &char)) -> bool {
    (*d.0 == 'M' || *d.0 == 'S') && (*d.1 == 'M' || *d.1 == 'S') && (*d.0 != *d.1)
}

#[derive(Clone, Copy)]
enum Direction {
    Back,
    Forward,
}

fn next_pos(pos: usize, len: usize, dir: Direction) -> Option<usize> {
    match dir {
        Direction::Back if pos > 0 => Some(pos - 1),
        Direction::Forward if pos < len - 1 => Some(pos + 1),
        _ => None,
    }
}
