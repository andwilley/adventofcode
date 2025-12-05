use std::io::{self, BufRead, Cursor};

pub fn find_doubles(reader: io::BufReader<Cursor<&str>>) -> io::Result<i64> {
    let mut ans = 0;
    for chunk_result in reader.split(b',') {
        let chunk = chunk_result?;
        let content =
            String::from_utf8(chunk).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let r = parse_range(&content.trim());
        if r.high.len() == r.low.len() && r.high.len() % 2 != 0 {
            // If they're the same length and odd we can skip
            continue;
        }
        let r = Range {
            low: r
                .low
                .parse::<i64>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
            high: r
                .high
                .parse::<i64>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
        };
        // first just loop over the intervals. probably a more clever way to do this.
        for x in r.low..=r.high {
            if is_double(&x.to_string()) {
                ans += x;
            }
        }
    }
    Ok(ans)
}

fn is_double(x: &str) -> bool {
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

fn parse_range(input: &str) -> Range<&str> {
    let (low, high) = input.split_once("-").unwrap();
    Range { low, high }
}

fn main() -> io::Result<()> {
    let input = include_str!("../data/input.txt");
    let reader = io::BufReader::new(Cursor::new(input));
    let ans = find_doubles(reader)?;
    println!("{ans}");
    return Ok(());
}

#[cfg(test)]
mod tests {
    use crate::find_doubles;
    use std::io::{self, Cursor};

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_fn() {
        let reader = io::BufReader::new(Cursor::new(TEST_INPUT));
        let ans = find_doubles(reader).expect("");
        assert_eq!(ans, 1227775554);
    }
}
