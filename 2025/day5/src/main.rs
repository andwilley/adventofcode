use rangemap::RangeInclusiveSet;
use std::{
    io::{self, BufRead, Cursor},
    ops::RangeInclusive,
};

pub(crate) fn freshness<F>(reader: io::BufReader<Cursor<&str>>, part: F) -> io::Result<u64>
where
    F: Fn(RangeInclusiveSet<i64>, Vec<i64>) -> u64,
{
    let mut rmap = RangeInclusiveSet::<i64>::new();
    let mut next = false;
    let mut inputs = Vec::<i64>::new();
    for line_result in reader.lines() {
        let line_untrimmed = line_result?;
        let line = line_untrimmed.trim();
        if line == "\n" || line == "" {
            next = true;
            continue;
        }
        if !next {
            let (s, e) = line
                .split_once("-")
                .map(|(f, l)| (f.parse().expect(""), l.parse().expect("")))
                .unwrap();
            rmap.insert(RangeInclusive::new(s, e))
        } else {
            inputs.push(line.parse().expect(""));
        }
    }
    Ok(part(rmap, inputs))
}

#[allow(dead_code)]
pub(crate) fn part_1(rmap: RangeInclusiveSet<i64>, inputs: Vec<i64>) -> u64 {
    let mut ans = 0;
    for i in inputs {
        if rmap.contains(&i) {
            ans += 1;
        }
    }
    ans
}

#[allow(dead_code)]
pub(crate) fn part_2(rmap: RangeInclusiveSet<i64>, _: Vec<i64>) -> u64 {
    let mut ans = 0;
    for r in rmap.iter() {
        ans += (r.end() - r.start()).abs() + 1;
    }
    ans as u64
}

fn main() -> io::Result<()> {
    let input = include_str!("../data/input.txt");
    let reader = io::BufReader::new(Cursor::new(input));
    let ans = freshness(reader, part_2)?;
    println!("{ans}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{freshness, part_1, part_2};
    use std::io::{self, Cursor};

    const TEST_INPUT: &str = "\
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32";

    #[test]
    fn part1() {
        let reader = io::BufReader::new(Cursor::new(TEST_INPUT));
        let ans = freshness(reader, part_1).expect("");
        assert_eq!(ans, 3);
    }

    #[test]
    fn part2() {
        let reader = io::BufReader::new(Cursor::new(TEST_INPUT));
        let ans = freshness(reader, part_2).expect("");
        assert_eq!(ans, 14);
    }
}
