fn main() {
    let (input_n, input_y) = input_n_y();
    let mut number_of_10000: i32 = -1;
    let mut number_of_5000: i32 = -1;
    let mut number_of_1000: i32 = -1;
    let mut sum = 0;

    'outer: for num_1000 in 0..input_n + 1 {
        for num_5000 in 0..(input_n + 1 - num_1000) {
            for num_10000 in 0..(input_n + 1 - num_1000 - num_5000) {
                let sum_yen = num_10000 * 10000 + num_5000 * 5000 + num_1000 * 1000;
                let sum_num = num_10000 + num_5000 + num_1000;
                if sum_yen == input_y {
                    number_of_10000 = num_10000 as i32;
                    number_of_5000 = num_5000 as i32;
                    number_of_1000 = num_1000 as i32;
                    break 'outer;
                }
            }
        }
    }
    println!("{} {} {}", number_of_10000, number_of_5000, number_of_1000);
}

////////////////////////////////////////////////////////////////////////////////

fn input_n_y() -> (u32, u32) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1]);
}
