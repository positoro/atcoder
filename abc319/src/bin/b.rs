fn main() {
    let n_u32: u32 = input_u32();
    let mut output_str_vec: Vec<u32> = Vec::new();
    output_str_vec.push(1);

    'outer: for i in 1..n_u32 {
        for j in 1..10 {
            if n_u32 % j == 0 {
                if i % (n_u32 / j) == 0 {
                    output_str_vec.push(j);
                    continue 'outer;
                }
            }
        }
        output_str_vec.push(0);
    }

    output_str_vec.push(1);
    for s in output_str_vec.iter() {
        if *s != 0 {
            print!("{}", s);
        } else {
            print!("-");
        }
    }
    println!("");
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
