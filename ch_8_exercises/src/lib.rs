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

use std::collections::HashMap;

pub struct Stats {
    pub median: Option<f64>,
    pub mode: Option<i32>,
}
pub fn find_median(list: &[i32]) -> Option<f64> {
    if list.is_empty() {
        return None;
    }
    let mut sorted = list.to_vec();
    sorted.sort();
    let length = sorted.len();
    let mid = length / 2;
    if length % 2 == 0 {
        Some((sorted[mid - 1] + sorted[mid]) as f64 / 2.0)
    } else {
        Some(sorted[mid] as f64)
    }
}

pub fn find_mode(list: &[i32]) -> Option<i32> {
    if list.is_empty() {
        return None;
    }
    let mut freqs = HashMap::new();
    let mut mode = list[0];
    let mut max = 0;

    for &num in list {
        let count = freqs.entry(num).or_insert(0);
        *count += 1;

        if *count > max {
            max = *count;
            mode = num;
        }
    }
    Some(mode)
}

pub fn find_median_mode(list: &[i32]) -> Stats {
    let median = find_median(list);
    let mode = find_mode(list);
    Stats { median, mode }
}
