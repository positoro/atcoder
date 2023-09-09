fn main() {
    let s_String: String = input_s();

    if s_String == "tourist" {
        println!("3858");
    } else if s_String == "ksun48" {
        println!("3679");
    } else if s_String == "Benq" {
        println!("3658");
    } else if s_String == "Um_nik" {
        println!("3648");
    } else if s_String == "apiad" {
        println!("3638");
    } else if s_String == "Stonefeang" {
        println!("3630");
    } else if s_String == "ecnerwala" {
        println!("3613");
    } else if s_String == "mnbvmar" {
        println!("3555");
    } else if s_String == "newbiedmy" {
        println!("3516");
    } else if s_String == "semiexp" {
        println!("3481");
    }
}

////////////////////////////////////////////////////////////////////////////////

fn input_s() -> String {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    input_strings.trim().parse().ok().unwrap()
}
