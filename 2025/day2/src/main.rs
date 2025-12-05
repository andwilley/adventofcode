use std::{
    collections::HashSet,
    io::{self, BufRead, Cursor},
};

pub fn find_invalid<F>(reader: io::BufReader<Cursor<&str>>, is_invalid: F) -> io::Result<i64>
where
    F: Fn(&str) -> bool,
{
    let mut ans = 0;
    for chunk_result in reader.split(b',') {
        let chunk = chunk_result?;
        let content =
            String::from_utf8(chunk).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let r = parse_range(content.trim())?;
        for x in r.low..=r.high {
            if is_invalid(&x.to_string()) {
                ans += x;
            }
        }
    }
    Ok(ans)
}

/**
 * Part 2
 *
 * After the first, for each char in the string that is the same as the first, consider everything
 * we've seen before that a possible repeater. Stack up these repeaters until we can invalidate
 * them via: at the end their length they don't devide into the string length or at any point they
 * don't continue to repeat. True (has duplicating pattern) means that there were repeaters left in
 * the set at the very end.
 */
pub fn is_duplicate(x: &str) -> bool {
    if x.len() < 2 {
        return false;
    }
    let first = &x[0..1];
    let mut possibles: HashSet<&str> = HashSet::new();
    possibles.insert(first);
    for i in 1..x.len() {
        let cur = &x[i..i + 1];
        if cur == first {
            let repeater = &x[..i];
            if x.len() % repeater.len() == 0 {
                possibles.insert(repeater);
            }
        }
        possibles.retain(|p| {
            // the index into p as if we kept repeating it
            let j = i % p.len();
            // if p[j] != current char, it doesn't continue to repeat, trash it
            *cur == p[j..j + 1]
        });
    }
    possibles.len() > 0
}

/** Part 1 */
pub fn is_double(x: &str) -> bool {
    if x.len() % 2 == 1 {
        return false;
    };
    let (start, end) = x.split_at(x.len() / 2);
    for (x, y) in start.chars().zip(end.chars()) {
        if x != y {
            return false;
        };
    }
    true
}

#[derive(Debug)]
struct Range<T> {
    low: T,
    high: T,
}

fn parse_range(input: &str) -> io::Result<Range<i64>> {
    let (low, high) = input.split_once("-").unwrap();
    let r = Range {
        low: low
            .parse::<i64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
        high: high
            .parse::<i64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
    };
    Ok(r)
}

fn main() -> io::Result<()> {
    let input = include_str!("../data/input.txt");
    let reader = io::BufReader::new(Cursor::new(input));
    let ans = find_invalid(reader, is_duplicate)?;
    println!("{ans}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{find_invalid, is_double, is_duplicate};
    use std::io::{self, Cursor};

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1() {
        let reader = io::BufReader::new(Cursor::new(TEST_INPUT));
        let ans = find_invalid(reader, is_double).expect("");
        assert_eq!(ans, 1227775554);
    }

    #[test]
    fn part2() {
        let reader = io::BufReader::new(Cursor::new(TEST_INPUT));
        let ans = find_invalid(reader, is_duplicate).expect("");
        assert_eq!(ans, 4174379265);
    }
}
