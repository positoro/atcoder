fn main() {
    let m_u32: u32 = input_u32();
    let s1_string: String = input_s();
    let s2_string: String = input_s();
    let s3_string: String = input_s();
    let mut print_time: i32 = m_u32 as i32 * 3;

    for i in 0..10 {
        let time: i32 = get_time(i, &s1_string, &s2_string, &s3_string);
        if time < print_time {
            print_time = time;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

fn get_time(n: u32, s1: &String, s2: &String, s3: &String) -> i32 {
    let time_of_s1: i32 = s1
        .find(std::char::from_digit(n, 10).unwrap())
        .unwrap_or(usize::MAX) as i32;
    let time_of_s2: i32 = s2
        .find(std::char::from_digit(n, 10).unwrap())
        .unwrap_or(usize::MAX) as i32;
    let time_of_s3: i32 = s3
        .find(std::char::from_digit(n, 10).unwrap())
        .unwrap_or(usize::MAX) as i32;

    let mut return_time: i32 = -1;
    if time_of_s1 != -1 && time_of_s2 == -1 && time_of_s3 != -1 {
        return_time = time_of_s3 + time_of_s2 + time_of_s1;
    }
    println!("{:?} {}", time_of_s1, n);
    println!("{:?} {}", time_of_s2, n);
    println!("{:?} {}", time_of_s3, n);

    return return_time;
}

fn input_s() -> String {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    input_strings.trim().parse().ok().unwrap()
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
