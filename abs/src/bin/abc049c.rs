fn main() {
    let mut input_string = input_string();
    if input_string.find("eraser").is_some() {
        input_string = input_string.replace("eraser", "");
    }
    if input_string.find("erase").is_some() {
        input_string = input_string.replace("erase", "");
    }
    if input_string.find("dreamer").is_some() {
        input_string = input_string.replace("dreamer", "");
    }
    if input_string.find("dream").is_some() {
        input_string = input_string.replace("dream", "");
    }

    let check_str = input_string.trim();
    if check_str.is_empty() {
        println!("YES");
    } else {
        println!("NO");
    }
}

////////////////////////////////////////////////////////////////////////////////

fn input_string() -> String {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    input_strings
}
