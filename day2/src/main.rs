fn main() {
    let file_content =
        std::fs::read_to_string("assets/second.txt").expect("Failed to read input file");

    let mut rotations = Vec::new();

    for line in file_content.lines() {
        rotations.push(parse_expression(line));
    }

    println!(
        "Sum of invalid IDs: {}",
        count_invalid_ids_in_ranges(rotations.concat())
    );

    println!(
        "Sum of invalid IDs (pattern): {}",
        count_invalid_ids_in_ranges_2(rotations.concat())
    );
}

#[derive(Clone)]
pub struct Range {
    pub start: i64,
    pub end: i64,
}

pub fn parse_expression(input: &str) -> Vec<Range> {
    let parts: Vec<&str> = input.split(',').collect();

    let mut ranges: Vec<Range> = Vec::new();

    for part in parts {
        let bounds: Vec<&str> = part.split('-').collect();
        if bounds.len() != 2 {
            panic!("Invalid range format");
        }

        ranges.push(Range {
            start: bounds[0].parse().expect("Failed to parse range start"),
            end: bounds[1].parse().expect("Failed to parse range end"),
        });
    }

    ranges
}

pub fn count_invalid_ids_in_ranges(ranges: Vec<Range>) -> i64 {
    let mut invalid_count: i64 = 0;

    for range in ranges {
        for i in range.start..=range.end {
            let id_str = i.to_string();
            let len = id_str.len();

            if len % 2 == 0 {
                let (first_half, second_half) = id_str.split_at(len / 2);
                if first_half == second_half {
                    invalid_count += i;
                }
            }
        }
    }

    invalid_count
}

pub fn count_invalid_ids_in_ranges_2(ranges: Vec<Range>) -> i64 {
    let mut invalid_count: i64 = 0;

    for range in ranges {
        for i in range.start..=range.end {
            let id_str = i.to_string();
            let len = id_str.len();

            for pat_len in 1..=(len / 2) {
                if len % pat_len != 0 {
                    continue;
                }

                let pattern = &id_str[0..pat_len];
                let repeat_count = len / pat_len;

                if pattern.repeat(repeat_count) == id_str {
                    invalid_count += i;
                    break;
                }
            }
        }
    }

    invalid_count
}
