/*

fn main() {
    let str = String::from("Rahul");
    let index = find_first_i(str);
    if index == -1 {
        println!("No 'i' found!");
    } else {
        println!("Index is: {}", index);
    }
}

fn find_first_i(string: String) -> i32 {
    for (index, char) in string.chars().enumerate() {
        if char == 'i' {
            return index as i32;
        }
    }
    return -1;
}

*/

fn main() {
    let str = String::from("Rahul");
    let index = find_first_i(str);
    match index {
        Option::Some(index) => println!("{}", index),
        Option::None => println!("No 'i' found!"),
    }
}

fn find_first_i(string: String) -> Option<i32> {
    for (index, char) in string.chars().enumerate() {
        if char == 'i' {
            return Some(index as i32);
        }
    }
    None
}
