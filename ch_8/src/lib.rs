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
use std::io;

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

pub fn convert_to_pig_latin(word: &str) -> String {
    //.chars() returns an iterator of over unicode scalar values, basically returns a data
    //structure that contains the characters that can be iterated on
    let mut chars = word.chars();

    let first_letter = match chars.next() {
        Some(c) => c,
        None => return String::new(),
    };

    //chars.collect() turns chars from an iterator back into a String and does so based on the type
    //declaration. Can turn other data streams into different types using collect, very powerful
    if "aeiouAEIOU".contains(first_letter) {
        format!("{word}-hay")
    } else {
        let rest: String = chars.collect();
        format!("{rest}-{first_letter}ay")
    }
}

pub fn add_employee_names() {
    println!("Employee Database Augmenter");
    println!("To add employee enter in format 'Add _name_ to _department_");
    println!("To retrieve all employees in a department type 'List _department_'");
    println!("To retrieve all employees in each department type 'All");

    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let command = input_data();
        let input: Vec<&str> = command.split_whitespace().collect();
        match input.as_slice() {
            ["Add", name, "to", department] => {
                employees
                    .entry(department.to_string())
                    .or_default()
                    .push(name.to_string());

                println!("{} added to {}", name, department);
            }
            ["List", department] => match employees.get(*department) {
                Some(names) => {
                    let mut sorted_names = names.clone();
                    sorted_names.sort();
                    println!("Employees in {}", department);
                    for name in sorted_names {
                        println!("{}", name)
                    }
                }
                None => {
                    println!("{} doesnt exist", department);
                }
            },
            ["All"] => {
                for (department, names) in &employees {
                    println!("{}:", department);
                    for name in names {
                        println!("{}", name);
                    }
                }
            }
            _ => {
                println!("Invalid command");
            }
        }
    }
}

fn input_data() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Cannot read name");
    let input = match input.trim().parse() {
        Ok(str) => str,
        Err(_) => String::from("Error"),
    };
    input
}
