fn main() {
    let mut p_vec_string: Vec<String> = input_string_vector_with_low(12);
    let mut first_polyomino: Vec<String> = Vec::new();
    let mut second_polyomino: Vec<String> = Vec::new();
    let mut third_polyomino: Vec<String> = Vec::new();

    third_polyomino.push(p_vec_string.pop().unwrap());
    third_polyomino.push(p_vec_string.pop().unwrap());
    third_polyomino.push(p_vec_string.pop().unwrap());
    third_polyomino.push(p_vec_string.pop().unwrap());
    third_polyomino.reverse();

    second_polyomino.push(p_vec_string.pop().unwrap());
    second_polyomino.push(p_vec_string.pop().unwrap());
    second_polyomino.push(p_vec_string.pop().unwrap());
    second_polyomino.push(p_vec_string.pop().unwrap());
    second_polyomino.reverse();

    first_polyomino.push(p_vec_string.pop().unwrap());
    first_polyomino.push(p_vec_string.pop().unwrap());
    first_polyomino.push(p_vec_string.pop().unwrap());
    first_polyomino.push(p_vec_string.pop().unwrap());
    first_polyomino.reverse();

    println!("{:?}", first_polyomino);
    println!("{:?}", second_polyomino);
    println!("{:?}", third_polyomino);
}

////////////////////////////////////////////////////////////////////////////////

fn input_string_vector_with_low(low: usize) -> Vec<String> {
    let mut input_strings = String::new();
    let mut return_vec: Vec<String> = Vec::new();

    for _ in 0..low {
        std::io::stdin().read_line(&mut input_strings).ok();
        let push_low: String = input_strings.trim().parse().ok().unwrap();
        return_vec.push(push_low);
        input_strings.clear();
    }

    return return_vec;
}
