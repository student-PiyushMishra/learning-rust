fn main() {
    let string = String::from("Piyush Mishra is a bad ass dev...");
    println!("{}", get_str_len(&string))
}

fn get_str_len(str: &str) -> usize {
    str.chars().count()
}
