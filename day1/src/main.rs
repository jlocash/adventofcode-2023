use std::fmt::Write;
use std::fs;
use std::ops::Index;
use std::path::Path;

fn to_number(s: &str) -> Option<u32> {
    if s.len() == 1 {
        return s.chars().next().and_then(|c| c.to_digit(10));
    }

    return match s {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    };
}

fn is_number_prefix(s: &str) -> bool {
    "zero".starts_with(s)
        || "one".starts_with(s)
        || "two".starts_with(s)
        || "three".starts_with(s)
        || "four".starts_with(s)
        || "five".starts_with(s)
        || "six".starts_with(s)
        || "seven".starts_with(s)
        || "eight".starts_with(s)
        || "nine".starts_with(s)
}

struct NumberIterator<'a> {
    s: &'a str,
    start_idx: usize,
}

impl<'a> NumberIterator<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            s,
            start_idx: 0,
        }
    }
}

impl<'a> Iterator for NumberIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // 5eightgdvgthfiveshthreesixfive
        // twone3four
        let mut stop_idx = self.start_idx;
        let mut num = None;
        while num.is_none() && self.start_idx < self.s.len() && stop_idx <= self.s.len() {
            let substr = &self.s[self.start_idx..stop_idx];
            num = to_number(substr);
            if num.is_some() || !is_number_prefix(substr) {
                self.start_idx += 1;
                stop_idx = self.start_idx;
            }
            stop_idx += 1;
        }
        return num;
    }
}


pub fn get_calibration_value(s: &str) -> u32 {
    let mut first_digit = None;
    let mut last_digit = None;

    NumberIterator::new(s).for_each(|n| {
        if first_digit.is_none() {
            first_digit = Some(n);
        }
        last_digit = Some(n);
    });

    let res = first_digit.unwrap() * 10 + last_digit.unwrap();
    return res;
}

#[cfg(test)]
mod test {
    #[test]
    fn test_number_iterator() {
        let tests = [
            ("5eightgdvgthfiveshthreesixfive", vec![5, 8, 5, 3, 6, 5]),
            ("twone3four", vec![2, 1, 3, 4]),
        ];

        tests.iter().for_each(|(s, res)| {
            let result: Vec<u32> = NumberIterator::new(s).collect();
            assert_eq!(result, *res);
        });
    }

    #[test]
    fn test_sum() {
        let lines = &[
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        assert_eq!(lines.map(get_calibration_value).iter().sum::<u32>(), 281);
    }
}

fn main() {
    let f = fs::read_to_string(Path::new("day1/input.txt"));
    match f {
        Ok(contents) => {
            let sum: u32 = contents
                .lines()
                .map(get_calibration_value)
                .sum();
            println!("Sum of all calibration values: {}", sum);
        }
        Err(e) => eprintln!("Error reading input.txt: {:?}", e)
    }
}
