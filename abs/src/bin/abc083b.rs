fn main() {
    let (input_n, input_a, input_b) = input_n_a_b();
    let mut counter: u64 = 0;
    if input_n != 10000 {
        for loop_num in 1..(input_n + 1) {
            let digit_1000 = loop_num / 1000;
            let digit_100 = (loop_num / 100) % 10;
            let digit_10 = (loop_num / 10) % 10;
            let digit_1 = loop_num % 10;
            let digit_number_sum = digit_1000 + digit_100 + digit_10 + digit_1;
            if input_a <= digit_number_sum && digit_number_sum <= input_b {
                counter += loop_num;
            }
        }
    } else {
        for loop_num in 1..10000 {
            let digit_1000 = loop_num / 1000 % 10;
            let digit_100 = (loop_num / 100) % 10;
            let digit_10 = (loop_num / 10) % 10;
            let digit_1 = loop_num % 10;
            let digit_number_sum = digit_1000 + digit_100 + digit_10 + digit_1;
            if input_a <= digit_number_sum && digit_number_sum <= input_b {
                counter += loop_num;
            }
        }
        counter += 10000;
    }

    println!("{}", counter);
}

////////////////////////////////////////////////////////////////////////////////

fn input_n_a_b() -> (u64, u64, u64) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u64> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1], v[2]);
}
