fn main() {
    let n_t: String = input_s();
    let n_t_vec: Vec<&str> = n_t.split_whitespace().collect::<Vec<&str>>();
    let n: u32 = n_t_vec[0].parse().unwrap();
    let s: Vec<String> = input_string_vector_with_low(n as usize);
    let mut print_vec: Vec<u32> = Vec::new();
    for i in 1..n + 1 {
        if check(&s[(i - 1) as usize], &n_t_vec[1]) == true {
            print_vec.push(i);
        }
    }
    println!("{:?}", print_vec.len());
    stdout_u32_vector(&print_vec);
}

////////////////////////////////////////////////////////////////////////////////

fn check(s: &str, t: &str) -> bool {
    if check_one(s, t) == true {
        return true;
    }

    if check_two(s, t) == true {
        return true;
    }

    if check_three(s, t) == true {
        return true;
    }

    if check_four(s, t) == true {
        return true;
    }

    return false;
}

fn check_one(s: &str, t: &str) -> bool {
    if s == t {
        return true;
    }
    return false;
}

fn check_two(s: &str, t: &str) -> bool {
    for i in 0..t.len() {
        let mut t_string: String = t.to_string();
        t_string.remove(i);
        if s == t_string {
            return true;
        }
    }
    return false;
}

fn check_three(s: &str, t: &str) -> bool {
    for i in 0..s.len() {
        let mut s_string: String = s.to_string();
        s_string.remove(i);
        if s_string == t {
            return true;
        }
    }
    return false;
}

fn check_four(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    for i in 0..s.len() {
        let mut s_string: String = s.to_string();
        let mut t_string: String = t.to_string();
        s_string.remove(i);
        t_string.remove(i);
        if s_string == t_string {
            return true;
        }
    }
    return false;
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

fn input_s() -> String {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    input_strings.trim().parse().ok().unwrap()
}

fn stdout_u32_vector(v: &Vec<u32>) {
    let s_vec: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    println!("{}", s_vec.join(" "));
}
