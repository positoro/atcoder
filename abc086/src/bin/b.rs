fn main() {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    input_strings.retain(|c| !c.is_whitespace());
    println!("{}", input_strings);
}
