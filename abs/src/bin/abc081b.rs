#[allow(unused_variables)]
fn main() {
    let input_n = input_n() as usize;
    let mut input_u32_vector = input_u32_vector();
    let mut counter: u32 = 0;

    'outer: loop {
        for num in input_u32_vector.iter() {
            if num % 2 == 1 {
                break 'outer;
            }
        }
        counter += 1;
        input_u32_vector = input_u32_vector.into_iter().map(|x| x / 2).collect();
    }

    println!("{}", counter);
}

////////////////////////////////////////////////////////////////////////////////

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

fn input_n() -> u8 {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u8> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v[0];
}
