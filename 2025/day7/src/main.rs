use std::io::{self, BufRead, Cursor};

pub(crate) fn teleport<F>(reader: io::BufReader<Cursor<&str>>, part: F) -> io::Result<u64>
where
    F: Fn() -> u64,
{
    let mut lines = reader.lines().peekable();
    while let Some(line_result) = lines.next() {
        let _ = line_result?;
    }
    Ok(part())
}

#[allow(dead_code)]
pub(crate) fn part_1() -> u64 {
    10
}

#[allow(dead_code)]
pub(crate) fn part_2() -> u64 {
    10
}

fn main() -> io::Result<()> {
    let input = include_str!("../data/input.txt");
    let reader = io::BufReader::new(Cursor::new(input));
    let ans = teleport(reader, part_2)?;
    println!("{ans}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{teleport, part_1, part_2};
    use std::io::{self, Cursor};

    const TEST_INPUT: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn part1() {
        let reader = io::BufReader::new(Cursor::new(TEST_INPUT));
        let ans = teleport(reader, part_1).expect("");
        assert_eq!(ans, 21);
    }

    #[test]
    fn part2() {
        let reader = io::BufReader::new(Cursor::new(TEST_INPUT));
        let ans = teleport(reader, part_2).expect("");
        assert_eq!(ans, 10);
    }
}
