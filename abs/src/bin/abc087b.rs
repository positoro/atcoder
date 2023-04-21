fn main() {
    let input_a = input_u16();
    let input_b = input_u16();
    let input_c = input_u16();
    let input_x = input_u16();
    let mut counter: u32 = 0;

    for number_of_500_yen in 0..input_a + 1 {
        for number_of_100_yen in 0..input_b + 1 {
            for number_of_50_yen in 0..input_c + 1 {
                if number_of_500_yen * 500 + number_of_100_yen * 100 + number_of_50_yen * 50
                    == input_x
                {
                    counter += 1;
                }
            }
        }
    }

    println!("{}", counter);
}

////////////////////////////////////////////////////////////////////////////////

fn input_u16() -> u16 {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u16> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v[0];
}
