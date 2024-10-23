////////////////////////////////////////////////////////////////////////////////

pub fn vector_vector_u32(v: &Vec<Vec<u32>>) {
    for h in v.iter() {
        for w in h.iter() {
            print!("{} ", w);
        }
        println!();
    }
}

pub fn i32_vector(v: &Vec<i32>) {
    let s_vec: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    println!("{}", s_vec.join(" "));
}
