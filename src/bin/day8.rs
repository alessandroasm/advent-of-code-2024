use std::collections::{HashMap, HashSet};

fn main() {
    let mut map = HashMap::new();
    let mut w = 0;
    let mut h = 0;

    for (y, line) in std::io::stdin().lines().enumerate() {
        let line = line.unwrap();
        h = y + 1;
        w = line.len();

        line.chars().enumerate().for_each(|(x, ch)| {
            if ch != '.' {
                let p = (x as i32, y as i32);
                map.entry(ch)
                    .and_modify(|v: &mut Vec<_>| v.push(p))
                    .or_insert_with(|| vec![p]);
            }
        });

        eprintln!("ln: {line}");
    }

    eprintln!("\nmap: {map:?}\nw: {w}, h: {h}");

    let mut antinodes = HashSet::new();
    let rw = 0..w as i32;
    let rh = 0..h as i32;

    let mut push_node = |p: (i32, i32)| {
        if rw.contains(&p.0) && rh.contains(&p.1) {
            antinodes.insert(p);
        }
    };

    for (_ch, entries) in map.into_iter() {
        let len = entries.len();
        if len < 2 {
            continue;
        }

        for i in 0..len - 1 {
            let p0 = &entries[i];

            for j in i + 1..len {
                let p1 = &entries[j];

                let d = (p1.0 - p0.0, p1.1 - p0.1);
                let a0 = (p0.0 - d.0, p0.1 - d.1);
                let a1 = (p1.0 + d.0, p1.1 + d.1);

                push_node(a0);
                push_node(a1);
            }
        }
    }

    eprintln!("Antinodes: {antinodes:?}");
    eprintln!("  cnt: {}", antinodes.len());
}
