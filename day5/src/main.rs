fn main() {
    let file_content =
        std::fs::read_to_string("assets/fifth.txt").expect("Failed to read input file");

    let (ranges, numbers) = parse_expression(&file_content);

    println!(
        "Number of non-spoiled numbers: {}",
        find_non_spoiled_numbers(&ranges, &numbers)
    );

    println!("Number of valid IDs: {}", get_number_of_valid_ids(&ranges));
}

pub fn parse_expression(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut sections = input.split("\n\n");
    let mut ranges = Vec::new();
    let mut numbers = Vec::new();

    for section in sections {
        for line in section.lines() {
            if line.contains('-') {
                let parts: Vec<&str> = line.split('-').collect();
                let start: i64 = parts[0].trim().parse().unwrap();
                let end: i64 = parts[1].trim().parse().unwrap();
                ranges.push((start, end));
            } else {
                let number: i64 = line.trim().parse().unwrap();
                numbers.push(number);
            }
        }
    }

    (ranges, numbers)
}

pub fn find_non_spoiled_numbers(ranges: &Vec<(i64, i64)>, numbers: &Vec<i64>) -> i64 {
    let mut total_non_spoiled: i64 = 0;

    for &number in numbers {
        for &(start, end) in ranges {
            if number >= start && number <= end {
                total_non_spoiled += 1;
                break;
            }
        }
    }

    total_non_spoiled
}

pub fn get_number_of_valid_ids(ranges: &Vec<(i64, i64)>) -> i64 {
    let mut count_of_valid_ids: i64 = 0;
    let mut sorted_ranges = ranges.clone();

    sorted_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    for i in 0..sorted_ranges.len() {
        let (start, end) = sorted_ranges[i];

        if i == 0 || start > sorted_ranges[i - 1].1 {
            count_of_valid_ids += end - start + 1;
        } else if end > sorted_ranges[i - 1].1 {
            count_of_valid_ids += end - sorted_ranges[i - 1].1;
        }
    }

    count_of_valid_ids
}
