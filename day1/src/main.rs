fn main() {
    let file_content =
        std::fs::read_to_string("assets/first.txt").expect("Failed to read input file");

    let mut rotations = Vec::new();

    for line in file_content.lines() {
        rotations.push(parse_expression(line));
    }

    println!(
        "Dial first password: {}",
        rotate_dial_count_null(50, Vec::clone(&rotations))
    );
    println!(
        "Dial second password: {}",
        rotate_dial_count_every_null(50, Vec::clone(&rotations))
    );
}

#[derive(Clone)]
pub enum RotationType {
    Left,
    Right,
}

pub fn parse_expression(input: &str) -> (RotationType, i32) {
    let rotation_type = match &input[0..1] {
        "L" => RotationType::Left,
        "R" => RotationType::Right,
        _ => panic!("Invalid rotation type"),
    };

    let rotation_value: i32 = input[1..].parse().expect("Failed to parse rotation value");

    (rotation_type, rotation_value)
}

pub fn rotate_dial_count_null(initial_position: i32, rotations: Vec<(RotationType, i32)>) -> i32 {
    let mut password = 0;
    let mut current_position = initial_position;

    for rotation in rotations {
        let rotation_value = rotation.1 % 100;

        match rotation.0 {
            RotationType::Left => {
                current_position -= rotation_value;
                if current_position < 0 {
                    current_position += 100;
                }
            }
            RotationType::Right => {
                current_position += rotation_value;
                if current_position >= 100 {
                    current_position -= 100;
                }
            }
        }

        if current_position == 0 {
            password += 1;
        }
    }

    password
}

pub fn rotate_dial_count_every_null(
    initial_position: i32,
    rotations: Vec<(RotationType, i32)>,
) -> i32 {
    let mut password = 0;
    let mut current_position = initial_position;

    for rotation in rotations {
        let rotation_value = rotation.1;

        match rotation.0 {
            RotationType::Left => {
                let mut steps = rotation_value;
                while steps > 0 {
                    if current_position == 0 {
                        password += 1;
                    }
                    current_position -= 1;
                    if current_position < 0 {
                        current_position = 99;
                    }
                    steps -= 1;
                }
            }
            RotationType::Right => {
                let mut steps = rotation_value;
                while steps > 0 {
                    if current_position == 0 {
                        password += 1;
                    }
                    current_position += 1;
                    if current_position >= 100 {
                        current_position = 0;
                    }
                    steps -= 1;
                }
            }
        }
    }

    password
}
