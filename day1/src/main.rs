use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
fn main() {
    let f = fs::File::open("input1.txt").unwrap();

    let depths: Vec<i32> = read_values(f).unwrap();

    let depth_window_sums = window_sum(&depths, 3);

    let increases = count_increases(&depths);
    let window_increases = count_increases(&depth_window_sums);
    println!("{}", increases);
    println!("{}", window_increases);
}

fn window_sum(values: &[i32], width: usize) -> Vec<i32> {
    let window_sums: Vec<i32> = values.windows(width).map(|vals| vals.iter().sum()).collect();
    window_sums
}

fn count_increases(depths: &[i32]) -> usize {
    depths
        .iter()
        .skip(1)
        .zip(depths.iter())
        .filter(|(&next, &prev)| is_increasing(prev, next))
        .count()
}

fn is_increasing(prev: i32, next: i32) -> bool {
    next > prev
}

fn read_values<R: Read>(io: R) -> Result<Vec<i32>, io::Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

#[test]
fn counts_increases() {
    let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    assert_eq!(count_increases(&depths), 7);
}

#[test]
fn window_sum_works() {
    
    let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    let window_sums = window_sum(&depths, 3);
    assert_eq!(count_increases(&window_sums), 5);
}