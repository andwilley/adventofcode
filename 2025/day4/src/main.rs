use std::{
    collections::HashMap,
    io::{self, BufRead, Cursor},
    ops::{Add, Div, Rem},
};

pub(crate) fn forklift<F>(reader: io::BufReader<Cursor<&str>>, part: F) -> io::Result<u64>
where
    F: Fn(Vec<Point<i32>>, &mut HashMap<Point<i32>, i32>, (i32, i32)) -> u64,
{
    let mut input: Vec<Point<i32>> = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    let mut index: i32 = -1;
    let mut rolls: HashMap<Point<i32>, i32> = HashMap::new();
    for (line_no, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        height += 1;
        if line_no == 0 {
            width = line.len();
        }
        let trimmed = line.trim();
        let points: Vec<Point<i32>> = trimmed
            .chars()
            .filter_map(|c| {
                if c == '@' {
                    index += 1;
                    Some(Point::from_index(index, width as i32))
                } else {
                    None
                }
            })
            .collect();
        let points: Vec<Point<i32>> = points
            .iter()
            .map(|roll| {
                let count = EIGHT_CELLS
                    .iter()
                    .filter_map(|c| {
                        validate_grid(*roll + *c, (height as i32, width as i32))
                            .map(|y| points[to_index(y, width as i32)])
                    })
                    .count();
                rolls.insert(*roll, count as i32);
                PointCount { p: *roll, count }
            })
            .filter(|p| p.count < 4)
            .map(|p| p.p)
            .collect();
        input.extend(points);
    }
    Ok(part(input, &mut rolls, (height as i32, width as i32)))
}

pub(crate) fn part_1(
    removes: Vec<Point<i32>>,
    _: &mut HashMap<Point<i32>, i32>,
    _: (i32, i32),
) -> u64 {
    removes.len() as u64
}

pub(crate) fn part_2(
    removes: Vec<Point<i32>>,
    rolls: &mut HashMap<Point<i32>, i32>,
    (h, w): (i32, i32),
) -> u64 {
    10
}

const EIGHT_CELLS: [Point<i32>; 8] = [
    Point { y: 1, x: 0 },   // down
    Point { y: -1, x: 0 },  // up
    Point { y: 0, x: 1 },   // right
    Point { y: 0, x: -1 },  // left
    Point { y: 1, x: 1 },   // down right
    Point { y: 1, x: -1 },  // down left
    Point { y: -1, x: 1 },  // up right
    Point { y: -1, x: -1 }, // up left
];

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point<T> {
    y: T,
    x: T,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct PointCount<T> {
    p: Point<T>,
    count: usize,
}

impl<T: Copy + Div<Output = T> + Rem<Output = T>> Point<T> {
    pub(crate) fn from_index(i: T, w: T) -> Self {
        to_grid(i, w)
    }
}

impl<T: Copy + Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    #[inline]
    fn add(self, rhs: Point<T>) -> Point<T> {
        Point {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

/// Assumes a valid grid within the index.
fn to_index(p: Point<i32>, w: i32) -> usize {
    (p.y * w + p.x % w) as usize
}

/// (y, x)
fn to_grid<T: Copy + Div<Output = T> + Rem<Output = T>>(i: T, w: T) -> Point<T> {
    Point { y: i / w, x: i % w }
}

fn validate_grid(p: Point<i32>, (h, w): (i32, i32)) -> Option<Point<i32>> {
    if p.x >= w || p.x < 0 || p.y >= h || p.y < 0 {
        return None;
    }
    Some(p)
}

fn main() -> io::Result<()> {
    let input = include_str!("../data/input.txt");
    let reader = io::BufReader::new(Cursor::new(input));
    let ans = forklift(reader, part_2)?;
    println!("{ans}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{forklift, part_1, part_2};
    use std::io::{self, Cursor};

    const TEST_INPUT: &str = "\
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.";

    #[test]
    fn part1() {
        let reader = io::BufReader::new(Cursor::new(TEST_INPUT));
        let ans = forklift(reader, part_1).expect("");
        assert_eq!(ans, 13);
    }

    #[test]
    fn part2() {
        let reader = io::BufReader::new(Cursor::new(TEST_INPUT));
        let ans = forklift(reader, part_2).expect("");
        assert_eq!(ans, 43);
    }
}
