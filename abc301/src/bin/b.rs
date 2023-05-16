fn main() {
    let input_n = input_n() as usize;
    let mut input_i32_vector = input_i32_vector();
    let mut output_i32_vector: Vec<i32> = Vec::new();

    output_i32_vector.push(input_i32_vector[0]);
    for n in 1..input_n {
        if (input_i32_vector[n - 1] - input_i32_vector[n]).abs() == 1 {
            //            println!("one diff");
            output_i32_vector.push(input_i32_vector[n]);
        //            println!("--------{}", input_i32_vector[n]);
        } else if input_i32_vector[n - 1] < input_i32_vector[n] {
            //            println!("upper");
            for i in (input_i32_vector[n - 1] + 1)..(input_i32_vector[n] + 1) {
                output_i32_vector.push(i);
                //                println!("--------{}", i);
            }
        } else if input_i32_vector[n - 1] > input_i32_vector[n] {
            //            println!("lower");
            for i in (input_i32_vector[n]..input_i32_vector[n - 1]).rev() {
                output_i32_vector.push(i);
                //                println!("--------{}", i);
            }
        }
    }

    println!("{:?}", output_i32_vector);
    stdout_i32_vector(&output_i32_vector);
}

////////////////////////////////////////////////////////////////////////////////

fn stdout_i32_vector(v: &Vec<i32>) {
    let s_vec: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    println!("{}", s_vec.join(" "));
}

fn input_i32_vector() -> Vec<i32> {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<i32> = input_strings
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
