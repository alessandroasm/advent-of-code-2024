use std::io::BufRead;
use std::io::Lines;
use std::io::StdinLock;

pub struct LineIter {
    lines: Lines<StdinLock<'static>>,
}

impl LineIter {
    pub fn new() -> Self {
        let stdin = std::io::stdin().lock();
        let lines = stdin.lines();
        Self { lines: lines }
    }
}

impl std::iter::Iterator for LineIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|l| l.unwrap())
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
