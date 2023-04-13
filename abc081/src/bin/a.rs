fn main() {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();

    let strings: &str = input_strings.trim();
    let strings_iter = strings.chars();
    let mut counter = 0;

    for character in strings_iter {
        if character == '1' {
            counter = counter + 1;
        }
    }
    println!("{}", counter);
}
