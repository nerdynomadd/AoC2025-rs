fn main() {
    let file_content =
        std::fs::read_to_string("assets/third.txt").expect("Failed to read input file");

    let mut banks = Vec::new();

    for line in file_content.lines() {
        banks.push(parse_expression(line));
    }

    println!(
        "Total power from turning on batteries: {}",
        turn_on_batteries(banks.clone())
    );

    println!(
        "Total power from turning on batteries (12 digits): {}",
        turn_on_batteries_twelve(banks.clone())
    );
}

pub fn parse_expression(input: &str) -> Vec<i32> {
    let mut batteries: Vec<i32> = Vec::new();

    for c in input.chars() {
        if !c.is_digit(10) && c != ' ' {
            panic!("Invalid character in bank data");
        }

        batteries.push(c.to_digit(10).unwrap() as i32);
    }

    batteries
}

pub fn turn_on_batteries(banks: Vec<Vec<i32>>) -> i32 {
    let mut total_power = 0;

    for bank in banks {
        let mut best_tens: i32 = -1;
        let mut best_value: i32 = -1;

        for &d in &bank {
            if best_tens >= 0 {
                let candidate = best_tens * 10 + d;
                if candidate > best_value {
                    best_value = candidate;
                }
            }

            if d > best_tens {
                best_tens = d;
            }
        }

        total_power += best_value;
    }

    total_power
}

pub fn turn_on_batteries_twelve(banks: Vec<Vec<i32>>) -> i64 {
    let mut total_power = 0;

    for bank in banks {
        let n = bank.len();
        let k = 12usize;

        let mut remove_count = n - k;
        let mut stack: Vec<i32> = Vec::new();

        for &digit in &bank {
            while remove_count > 0 && !stack.is_empty() && *stack.last().unwrap() < digit {
                stack.pop();
                remove_count -= 1;
            }
            stack.push(digit);
        }

        stack.truncate(k);

        let mut value: i64 = 0;

        for &digit in &stack {
            value = value * 10 + digit as i64;
        }

        total_power += value;
    }

    total_power
}
