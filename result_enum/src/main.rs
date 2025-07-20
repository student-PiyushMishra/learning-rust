use std::fs::read_to_string;

fn main() {
    let output_of_file = read_to_string("./a"); // if file "a" is present, it logs its content else logs error
    match output_of_file {
        Ok(text) => println!("{}", text),
        Err(e) => println!("Error while reading the file... \n {}", e),
    };
}
