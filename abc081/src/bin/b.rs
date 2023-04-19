fn main() {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    input_strings.clear();
    std::io::stdin().read_line(&mut input_strings).ok();
    let strings_vec: Vec<&str> = input_strings.trim().split_whitespace().collect();

    let mut counter = 0;
    let mut counted_vec: Vec<u32> = Vec::new();
    let mut next_counted_vec: Vec<u32> = Vec::new();

    for number_char in strings_vec {
        counted_vec.push(number_char.parse().unwrap());
    }

    'outer: loop {
        for number in &counted_vec {
            if number % 2 == 0 {
                next_counted_vec.push(*number / 2);
            } else {
                break 'outer;
            }
        }

        counted_vec.clear();
        for number in &next_counted_vec {
            counted_vec.push(*number);
        }
        next_counted_vec.clear();

        counter = counter + 1;
    }
    println!("{}", counter);
}
