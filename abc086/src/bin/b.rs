fn main() {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    input_strings.retain(|c| !c.is_whitespace());
    let number: f32 = input_strings.parse().unwrap();
    let sqrt_of_number: f32 = number.sqrt();
    let zyosuu: f32 = number % sqrt_of_number;

    if zyosuu == 0.0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
