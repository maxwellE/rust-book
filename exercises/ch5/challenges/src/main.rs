use std::collections::HashMap;

// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

fn median_and_mode(vec: Vec<i32>) -> (i32, i32) {
    let mut mode_map: HashMap<i32, i32> = HashMap::new();
    let vec_len = vec.len();
    let mut median = 0;
    let mut sorted_vec = vec.clone();
    sorted_vec.sort();
    for (i, &e) in sorted_vec.iter().enumerate() {
        let element_count: i32 = *mode_map.entry(e).or_insert(0);
        mode_map.insert(e, element_count + 1);
        let mut is_odd = true;
        if vec_len.is_multiple_of(2) {
            is_odd = false;
        }
        if is_odd {
            if i == vec_len / 2 {
                median = e;
            }
        } else if i == (vec_len / 2) + 1 {
            median = e;
        }
    }
    let mut highest_key = 0;
    let mut highest_value = 0;
    for (key, value) in &mode_map {
        if *value > highest_value {
            highest_value = *value;
            highest_key = *key;
        }
    }
    println!("mode_map: {:?}", mode_map);
    (median, highest_key)
}

// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!

fn pig_latin(str: &str) -> String {
    const CONSONANTS: &str = "BCDFGHJKLMNPQRSTVWXYZ";
    let mut result = String::new();
    for word in str.split_whitespace() {
        if let Some(first_char) = word.chars().next() {
            if CONSONANTS.contains(first_char.to_ascii_uppercase()) {
                for (i, word_char) in word.chars().enumerate() {
                    if i > 0 {
                        result.push(word_char);
                    }
                }
                result.push('-');
                result.push(first_char);
                result.push_str("ay");
                result.push(' ');
            } else {
                result.push_str(word);
                result.push_str("-hay ");
            }
        }
    }
    result
}

// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

struct Company {
    deparments: HashMap<String, Vec<String>>
}

impl Company {
    fn new() -> Company {
        Company {
            deparments: HashMap::new()
        }
    }

    fn add_employee(&mut self, command: &str) {
        let mut words = command.split_whitespace();
        if let Some(employee) = words.nth(1) &&
            let Some(deparment) = words.nth(1) {
                self.deparments
                    .entry(deparment.to_string())
                    .and_modify(|e| {
                        e.push(employee.to_string());
                        e.sort()
                    })
                    .or_insert(vec![employee.to_string()]);
            }
    }

    fn employees_in_department(&self, deparment: &str) -> String {
        let mut result = String::new();
        if let Some(employees) = self.deparments.get(deparment) {
            for employee in employees {
                result.push_str(employee);
                result.push_str("\n")
            }
        }
        result
    }

    fn employees_by_department(&self) -> String {
        let mut result = String::new();
        for (deparment, employees) in self.deparments.iter() {
            result.push_str(deparment);
            result.push_str(": ");
            for employee in employees {
                result.push_str(employee);
                result.push_str(", ")
            }
            result.push_str("\n")
        }
        result
    }
}


fn main() {
    let vec = vec![1, 2, 3, 4, 5, 5, 5];
    let (median, mode) = median_and_mode(vec);
    println!("median is: {}", median);
    println!("mode is: {}", mode);
    println!("first in pig latin is: {}", pig_latin("first"));
    let mut company: Company = Company::new();
    company.add_employee("Add Sally to Engineering");
    company.add_employee("Add Betty to Sales");
    company.add_employee("Add Amir to Sales");
    println!("Employess in Sales:\n{}", company.employees_in_department("Sales"));
    println!("Employess by department:\n{}", company.employees_by_department());
}
