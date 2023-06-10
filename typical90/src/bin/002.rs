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

fn get_bit(value: u32, digit: u32) -> u32 {
    (value >> (digit - 1)) & 1
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

fn create_parenthes(value: u32, n: u32) -> String {
    let mut return_string: String = String::new();

    for i in (1..(n + 1)).rev() {
        if get_bit(value, i) & 1 == 0 {
            return_string.push('(');
        } else {
            return_string.push(')');
        }
    }

    return return_string;
}

fn check_parenthes(value: u32, n: u32) -> bool {
    let mut bit_counter: u32 = 0;
    let mut no_bit_counter: u32 = 0;

    if count_bit(value) + count_bit(value) != n {
        return false;
    }

    for i in (1..(n + 1)).rev() {
        if get_bit(value, i) == 0 {
            no_bit_counter = no_bit_counter + 1;
        } else {
            bit_counter = bit_counter + 1;
        }

        if no_bit_counter < bit_counter {
            return false;
        }
    }

    return true;
}

fn get_parentheses(n: u32) -> Vec<String> {
    let mut return_vec_string: Vec<String> = Vec::new();
    let check_max = 2u32.pow(n - 1);

    for i in 0..check_max {
        if check_parenthes(i, n) == true {
            return_vec_string.push(create_parenthes(i, n));
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
