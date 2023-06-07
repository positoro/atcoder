fn main() {
    let input_u32_n = input_u32();

    if input_u32_n % 2 == 0 {
        let parentheses: Vec<String> = get_parentheses(input_u32_n);
        stdout_string_vector(&parentheses);
    }
}

////////////////////////////////////////////////////////////////////////////////

fn stdout_string_vector(v: &Vec<String>) {
    for val in v.iter() {
        println!("{}", val);
    }
}

fn count_bit(value: u32) -> u32 {
    let mut input_value: u32 = value;
    let mut return_value: u32 = 0;
    while input_value > 0 {
        return_value = return_value + (input_value & 1);
        input_value = input_value >> 1;
    }

    return return_value;
}

fn get_bit_length(value: u32) -> u32 {
    let length_in_bit: u32 = value.checked_ilog2().unwrap() + 1;

    return length_in_bit;
}

fn count_no_bit(value: u32) -> u32 {
    let return_value: u32 = get_bit_length(value) - count_bit(value);

    return return_value;
}

fn create_parenthes(value: u32) -> String {
    let mut return_string: String = String::new();
    let mut input_value: u32 = value;

    while input_value > 0 {
        if (input_value & 1) == 0 {
            return_string.push('(');
        } else {
            return_string.push(')');
        }
        input_value = input_value >> 1;
    }

    return return_string;
}

fn check_parenthes(value: u32) -> bool {
    let mut return_bool: bool = true;
    let mut input_value: u32 = value;
    let mut bit_counter: u32 = 0;
    let mut no_bit_counter: u32 = 0;

    if count_bit(value) != count_no_bit(value) {
        return_bool = false;
        return return_bool;
    }

    while input_value > 0 {
        if (input_value & 1) == 0 {
            no_bit_counter = no_bit_counter + 1;
        } else {
            bit_counter = bit_counter + 1;
        }

        if no_bit_counter < bit_counter {
            return_bool = false;
            break;
        }

        input_value = input_value >> 1;
    }
    return return_bool;
}

fn get_parentheses(n: u32) -> Vec<String> {
    let mut return_vec_string: Vec<String> = Vec::new();

    for i in 2_u32.pow(n - 1)..(2_u32.pow(n) - 1) {
        if check_parenthes(i) == true {
            return_vec_string.insert(0, create_parenthes(i));
        }
    }

    return return_vec_string;
}

fn input_u32() -> u32 {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v[0];
}
