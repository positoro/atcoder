#[allow(unused_variables)]
fn main() {
    let (input_n, input_k) = input_n_k();
    let input_a = input_a();
    let mut deduped_a: Vec<u32> = input_a.clone();
    let mut counter_a: Vec<u32> = Vec::new();
    let mut output_counter: u32 = 0;

    deduped_a.sort();
    deduped_a.dedup();

    for filter in deduped_a.iter() {
        counter_a.push(input_a.iter().filter(|&n| *n == *filter).count() as u32);
    }
    counter_a.sort();
    let counter_slice: &[u32] = &counter_a[0..counter_a.len() - input_k as usize];
    println!("-------------{:?}", counter_a);
    println!("-------------{:?}", counter_slice);
    println!("{}", output_counter);
}

////////////////////////////////////////////////////////////////////////////////

fn input_n_k() -> (u32, u32) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1]);
}

fn input_a() -> Vec<u32> {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v;
}
