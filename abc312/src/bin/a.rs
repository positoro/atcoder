fn main() {
    let s_string = input_s();
    if s_string == "ACE"
        || s_string == "BDF"
        || s_string == "CEG"
        || s_string == "DFA"
        || s_string == "EGB"
        || s_string == "FAC"
        || s_string == "GBD"
    {
        println!("Yes");
    } else {
        println!("No");
    }
}

////////////////////////////////////////////////////////////////////////////////

fn input_s() -> String {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    input_strings.trim().parse().ok().unwrap()
}
