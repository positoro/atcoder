fn main() {
    let s_u32_vec: Vec<u32> = input_u32_vector();

    if check_25(&s_u32_vec) && check_100_675(&s_u32_vec) && check_ascent(&s_u32_vec) {
        println!("Yes");
    } else {
        println!("No");
    }
}

////////////////////////////////////////////////////////////////////////////////

fn check_25(s_u32_vec: &Vec<u32>) -> bool {
    for s in s_u32_vec.iter() {
        if *s % 25 != 0 {
            return false;
        }
    }
    return true;
}

fn check_100_675(s_u32_vec: &Vec<u32>) -> bool {
    for s in s_u32_vec.iter() {
        if *s > 675 || *s < 100 {
            return false;
        }
    }
    return true;
}

fn check_ascent(s_u32_vec: &Vec<u32>) -> bool {
    let mut last: u32 = 0;
    for s in s_u32_vec.iter() {
        if last > *s {
            return false;
        }
        last = *s
    }
    return true;
}

fn input_u32_vector() -> Vec<u32> {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v;
}
