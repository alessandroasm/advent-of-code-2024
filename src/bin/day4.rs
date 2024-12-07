fn main() {
    let map: Vec<Vec<_>> = advent_of_code_2024::LineIter::new()
        .map(|ln| ln.chars().collect())
        .collect();

    let directions = [
        (Direction::None, Direction::Back),
        (Direction::Forward, Direction::None),
        (Direction::None, Direction::Forward),
        (Direction::Back, Direction::None),
        (Direction::Forward, Direction::Back),
        (Direction::Forward, Direction::Forward),
        (Direction::Back, Direction::Forward),
        (Direction::Back, Direction::Back),
    ];

    let mut cnt = 0;

    map.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, ch)| {
            if *ch == 'X' {
                directions.iter().for_each(|(dh, dv)| {
                    if find_word(&map, x, y, *dh, *dv) {
                        cnt += 1;
                    }
                });
            }
        });
    });

    eprintln!("Count: {cnt}");
}

#[derive(Clone, Copy)]
enum Direction {
    None,
    Back,
    Forward,
}

fn next_pos(pos: usize, len: usize, dir: Direction) -> Option<usize> {
    match dir {
        Direction::None => Some(pos),
        Direction::Back if pos > 0 => Some(pos - 1),
        Direction::Forward if pos < len - 1 => Some(pos + 1),
        _ => None,
    }
}

fn find_word(map: &Vec<Vec<char>>, x: usize, y: usize, dir_h: Direction, dir_v: Direction) -> bool {
    let len_x = map[0].len();
    let len_y = map.len();

    let mut px = x;
    let mut py = y;
    for ch in ['M', 'A', 'S'] {
        let Some(nx) = next_pos(px, len_x, dir_h) else {
            return false;
        };
        let Some(ny) = next_pos(py, len_y, dir_v) else {
            return false;
        };

        px = nx;
        py = ny;

        if map[py][px] != ch {
            return false;
        }
    }

    true
}
