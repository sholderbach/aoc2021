use std::{fs, panic, vec};

fn main() {
    let data = fs::read_to_string("input").unwrap();
    let power = power_consumption(&data);
    let scrubber = scrubber_rating(&data);
    println!("{}", power);
    println!("{}", scrubber);
}

fn gamma_rate(data: &[Vec<bool>]) -> i64 {
    bits_to_digit(&tally_bits(data))
}

fn epsilon_rate(data: &[Vec<bool>]) -> i64 {
    let bits: Vec<_> = tally_bits(data).iter().map(|x| !x).collect();
    bits_to_digit(&bits)
}

fn power_consumption(data: &str) -> i64 {
    let bit_array = read_byte_arr(data);

    gamma_rate(&bit_array) * epsilon_rate(&bit_array)
}

fn scrubber_rating(data: &str) -> i64 {
    let data = read_byte_arr(data);
    let o2 = filter_rating(&data, o2_bit);
    let co2 = filter_rating(&data, co2_bit);

    o2 * co2
}

fn filter_rating(data: &[Vec<bool>], bit_filter: fn(&[bool]) -> bool) -> i64 {
    let t = transpose(data.to_owned());
    let len = t.len();

    let mut mask = vec![true; data.len()];

    let mut res_bits: Vec<bool> = Vec::with_capacity(len);
    for col in t {
        let allowed_bits = col
            .iter()
            .zip(mask.iter())
            .filter(|(_, m)| **m)
            .map(|(x, _)| *x)
            .collect::<Vec<_>>();
        if allowed_bits.len() > 1 {
            let filter_cond = bit_filter(&allowed_bits);
            res_bits.push(filter_cond);

            for (m, x) in mask.iter_mut().zip(col) {
                *m = *m && (x == filter_cond);
            }
        } else {
            res_bits.push(allowed_bits[0])
        }
    }
    bits_to_digit(&res_bits)
}

fn char_to_bit(char: char) -> bool {
    match char {
        '0' => false,
        '1' => true,
        _ => panic!("Unexpected character"),
    }
}

fn read_byte_arr(str: &str) -> Vec<Vec<bool>> {
    str.lines()
        .map(|line| line.trim().chars().map(char_to_bit).collect())
        .collect()
}

fn tally_bits(data: &[Vec<bool>]) -> Vec<bool> {
    let data = data.to_owned();

    let t = transpose(data);
    t.iter().map(|x| majority_bit(x)).collect()
}

fn majority_bit(data: &[bool]) -> bool {
    let depth = data.len();
    let bit_count = data.iter().filter(|x| **x).count();
    bit_count * 2 > depth
}

fn o2_bit(data: &[bool]) -> bool {
    let depth = data.len();
    let bit_count = data.iter().filter(|x| **x).count();
    bit_count * 2 >= depth
}
fn co2_bit(data: &[bool]) -> bool {
    let depth = data.len();
    let bit_count = data.iter().filter(|x| **x).count();
    bit_count * 2 < depth
}

fn bits_to_digit(data: &[bool]) -> i64 {
    let mut out = 0;

    for &bit in data {
        out *= 2;
        out += if bit { 1 } else { 0 };
    }

    out
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut flattened: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            flattened
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
const TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

#[test]
fn test_powerconsumption() {
    assert_eq!(power_consumption(TEST_INPUT), 198)
}

#[test]
fn test_gamma_rate() {
    let res = gamma_rate(&read_byte_arr(TEST_INPUT));
    assert_eq!(res, 22);
}

#[test]
fn test_epsilon_rate() {
    let res = epsilon_rate(&read_byte_arr(TEST_INPUT));
    assert_eq!(res, 9);
}

#[test]
fn test_o2_rating() {
    let res = filter_rating(&read_byte_arr(TEST_INPUT), o2_bit);
    assert_eq!(res, 23);
}

#[test]
fn test_co2_rating() {
    let res = filter_rating(&read_byte_arr(TEST_INPUT), co2_bit);
    assert_eq!(res, 10);
}

#[test]
fn test_scrubber_rating() {
    assert_eq!(scrubber_rating(TEST_INPUT), 230);
}

#[test]
fn bit_conversion() {
    let dummy = vec![true, false, true, true];

    assert_eq!(bits_to_digit(&dummy), 11);
}

#[test]
fn test_co2bit() {
    let dummy = vec![true, true, false, false];
    assert_eq!(co2_bit(&dummy), false);
    let dummy2 = vec![true, true, true, false];
    assert_eq!(co2_bit(&dummy2), false);
    let dummy3 = vec![true, false, false, false];
    assert_eq!(co2_bit(&dummy3), true);
}
