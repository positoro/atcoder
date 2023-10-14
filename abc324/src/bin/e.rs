fn main() {
    let n_l_tuple_u32: (u32, u32) = input_tuple();
    let k_u32 = input_u32();
    let a_vec_u32 = input_vector_u32();

    println!("{:?}", n_l_tuple_u32);
    println!("{:?}", k_u32);
    println!("{:?}", a_vec_u32);
}

////////////////////////////////////////////////////////////////////////////////

fn _binary_search_check_satisfaction(vector: &Vec<u32>, index: usize, value: u32) -> bool {
    if value <= vector[index] {
        return true;
    } else {
        return false;
    }
}

fn binary_search(a: &Vec<u32>, b: u32) -> usize {
    let mut left: i32 = -1;
    let mut right: i32 = a.len() as i32;

    while (left + 1) < right {
        let mid: i32 = (left + right) / 2;
        if _binary_search_check_satisfaction(&a, mid as usize, b) == true {
            right = mid;
        } else {
            left = mid;
        }
    }
    return right as usize;
}

fn input_vector_u32_low(low: &u32) -> Vec<u32> {
    let mut return_vector: Vec<u32> = Vec::new();

    for _ in 0..*low {
        return_vector.push(input_u32());
    }

    return return_vector;
}

fn stdout_vector_vector_u32(v: &Vec<Vec<u32>>) {
    for h in v.iter() {
        for w in h.iter() {
            print!("{} ", w);
        }
        println!();
    }
}

fn input_u32_vector_vector(low: usize) -> Vec<Vec<u32>> {
    let mut input_strings = String::new();
    let mut return_u32_vector_vector = Vec::new();
    for _ in 0..low {
        std::io::stdin().read_line(&mut input_strings).ok();
        let v: Vec<u32> = input_strings
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();
        input_strings.clear();
        return_u32_vector_vector.push(v);
    }

    return return_u32_vector_vector;
}

fn get_index_of_equal_string_in_string_vector(v_s: &Vec<String>, s: &String) -> i32 {
    let mut return_index: i32 = -1;

    for (j, d) in v_s.iter().enumerate() {
        if *d == *s {
            return_index = j as i32;
            break;
        }
    }
    return return_index;
}

fn input_string_vector() -> Vec<String> {
    let mut input_strings = String::new();
    let mut return_vector: Vec<String> = Vec::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<&str> = input_strings.split_whitespace().collect();
    for s in v.iter() {
        return_vector.push(s.to_string());
    }

    return return_vector;
}

fn input_u32_tuple_vector(n: u32) -> Vec<(u32, u32)> {
    let mut input_strings = String::new();
    let mut return_vector = Vec::new();

    for _ in 0..(n - 1) {
        std::io::stdin().read_line(&mut input_strings).ok();
        let v: Vec<u32> = input_strings
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();
        input_strings.clear();
        return_vector.push((v[0], v[1]));
    }

    return return_vector;
}

fn get_char_from_string_vector(h: usize, w: usize, string_vector: &Vec<String>) -> char {
    let return_char: char = string_vector[h].chars().nth(w).unwrap();
    return return_char;
}

fn input_string_vector_with_low(low: usize) -> Vec<String> {
    let mut input_strings = String::new();
    let mut return_vec: Vec<String> = Vec::new();

    for _ in 0..low {
        std::io::stdin().read_line(&mut input_strings).ok();
        let push_low: String = input_strings.trim().parse().ok().unwrap();
        return_vec.push(push_low);
        input_strings.clear();
    }

    return return_vec;
}

fn input_tuple_low(low: &u32) -> Vec<(u32, u32)> {
    let mut return_vector: Vec<(u32, u32)> = Vec::new();

    for _ in 0..*low {
        return_vector.push(input_tuple());
    }

    return return_vector;
}

fn input_tuple() -> (u32, u32) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1]);
}

fn stdout_i32_vector(v: &Vec<i32>) {
    let s_vec: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    println!("{}", s_vec.join(" "));
}

fn input_s() -> String {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    input_strings.trim().parse().ok().unwrap()
}

fn input_vector_u32() -> Vec<u32> {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v;
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
