use std::{fs::read_to_string};

fn main() {
    let result = read_to_string("./a.txt");
    match result {
        Ok(output) => println!("{}",output),
        Err(error) => println!("Error while reading the file! \n {}",error)
    }    
}
