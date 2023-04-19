#[allow(unused_variables)]
fn main() {
    let (input_n, input_k) = input_n_k();
    let input_a = input_a();
    let mut deduped_a: Vec<u16> = input_a.clone();
    let mut counter_a: Vec<u16> = Vec::new();
    let mut output_counter: u16 = 0;

    deduped_a.sort();
    deduped_a.dedup();

    for filter in deduped_a.iter() {
        counter_a.push(input_a.iter().filter(|&n| *n == *filter).count() as u16);
    }
    counter_a.sort();
    counter_a.reverse();

    loop {
        if input_k < counter_a.len() as u16 {
            output_counter = output_counter + counter_a.pop().unwrap();
        } else {
            break;
        }
    }

    println!("{}", output_counter);
}

////////////////////////////////////////////////////////////////////////////////

fn input_n_k() -> (u16, u16) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u16> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1]);
}

fn input_a() -> Vec<u16> {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u16> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v;
}
