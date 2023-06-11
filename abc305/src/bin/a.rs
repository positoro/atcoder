fn main() {
    let n: i32 = input_u32() as i32;
    let mut distance: Vec<(i32, i32)> = Vec::new();
    for i in (0..101).step_by(5) {
        distance.push(((n - i).abs(), i));
    }
    let mut min_distance: i32 = 100;
    let mut min_index: usize = 0;

    for (index, value) in distance.iter().enumerate() {
        if value.0 < min_distance {
            min_distance = value.0;
            min_index = index;
        }
    }

    println!("{:?}", distance[min_index as usize].1);
}

////////////////////////////////////////////////////////////////////////////////

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
