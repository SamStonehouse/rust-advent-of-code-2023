use crate::inputs;

const INITIAL_VALID_STRINGS: [&str; 10] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
];

const VALID_STRINGS: [&str; 20] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn map_to_int_value(val: &str) -> Option<u32> {
    match val {
        "0" => Some(0),
        "1" => Some(1),
        "2" => Some(2),
        "3" => Some(3),
        "4" => Some(4),
        "5" => Some(5),
        "6" => Some(6),
        "7" => Some(7),
        "8" => Some(8),
        "9" => Some(9),
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
    }
}

fn find_first_match(str: &String, values: Vec<&str>) -> Option<String> {
    // Iterate through slices of the whole string, from the first letter, increasing in
    // size each iteration until a match is found with one of the values
    let mut index: usize = 1;
    while index <= str.len() {
        for v in values.clone().into_iter() {
            let (first, _) = str.split_at(index);
            if first.contains(v) {
                return Some(v.to_string());
            }
        }
        index += 1;
    }

    return None;
}

fn reverse_str(str: &str) -> String {
    str.chars().rev().collect()
}

fn find_last_match(str: &String, values: Vec<&str>) -> Option<String> {
    // Reverse all strings then do the normal forward match and reverse the found string
    let reversed_value_strings: Vec<String> = values.iter().map(|val| reverse_str(val)).collect();
    let reversed_values: Vec<&str> = reversed_value_strings
        .iter()
        .map(|val| val.as_str())
        .collect();
    let matched_str = find_first_match(&reverse_str(str), reversed_values);
    match matched_str {
        Some(val) => Some(reverse_str(&val)),
        None => None,
    }
}

fn parse_configuration_value(configuration_string: &String, valid_strings: Vec<&str>) -> Option<u32> {
    let first = map_to_int_value(&find_first_match(configuration_string, valid_strings.clone())?);
    let last = map_to_int_value(&find_last_match(configuration_string, valid_strings.clone())?);

    return Some(first? * 10 + last?);
}

pub(crate) fn part_one() {
    println!("Day One, Part One");
    let lines = inputs::read_inputs_from_file("./inputs/day_one.txt").unwrap();
    let valid_strings = Vec::from(INITIAL_VALID_STRINGS);
    let total: u32 = lines
        .iter()
        .map(|line| parse_configuration_value(line, valid_strings.clone()))
        .map(|opt| opt.unwrap())
        .sum();
    println!("Total sum: {:}", total);
}

pub(crate) fn part_two() {
    println!("Day One, Part Two");
    let lines = inputs::read_inputs_from_file("./inputs/day_one.txt").unwrap();
    let valid_strings = Vec::from(VALID_STRINGS);
    let total: u32 = lines
        .iter()
        .map(|replaced_str| parse_configuration_value(&replaced_str, valid_strings.clone()))
        .map(|opt| opt.unwrap())
        .sum();
    println!("Total sum: {:}", total);
}
