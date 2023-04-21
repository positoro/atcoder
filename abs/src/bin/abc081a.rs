fn main() {
    let input_s = input_s();

    println!("{}", input_s.match_indices("1").count());
}

////////////////////////////////////////////////////////////////////////////////

fn input_s() -> String {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    return input_strings;
}
