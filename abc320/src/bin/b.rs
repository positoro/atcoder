fn main() {
    let s_string: String = input_s();
    for n in 0..s_string.len() {
        for i in 0..n + 1 {
            println!("{:?} {}", n, i);
            check_palindrome(&s_string[i..s_string.len() - n + i]);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
fn check_palindrome(s: &str) -> bool {
    println!("{:?}", s);
    false
}

fn input_s() -> String {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    input_strings.trim().parse().ok().unwrap()
}
