fn main() {
    let n: u32 = input_u32();
    let a_u32_vec: Vec<u32> = input_u32_vector();
    let mut answer_vec: Vec<u32> = Vec::new();

    for i in 0..n {
        let mut sum = 0;
        for d in 0..7 {
            sum += a_u32_vec[(i * 7 + d) as usize];
        }
        answer_vec.push(sum);
    }

    stdout_u32_vector(&answer_vec);
}

////////////////////////////////////////////////////////////////////////////////

fn stdout_u32_vector(v: &Vec<u32>) {
    let s_vec: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    println!("{}", s_vec.join(" "));
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
