use std::io;

fn main() {
    let name: String = get_user_intro();
    greet(&name);
    loop {
        let mut op: String = String::from("");
        println!("Choose an operation to perform:- (+,-,*,/) or press 'NO' to end the program...");
        io::stdin()
            .read_line(&mut op)
            .expect("Failed to get the operation!");
        let op: &str = op.trim();
        if op == "NO" {
            println!("Calc.io signing off!");
            std::process::exit(0)
        }
        let x: f32 = get_num("Enter num1:-");
        let y: f32 = get_num("Enter num2:-");
        let result: f32 = match op {
            "+" => x + y,
            "-" => x - y,
            "*" => x * y,
            "/" => {
                if y == 0.0 {
                    return println!("Division by zero is NOT allowed!");
                }
                x / y
            }
            _ => {
                return println!("Invalid operator: {}", op);
            }
        };
        println!("The resultant is: {}", result)
    }
}

fn get_user_intro() -> String {
    println!("Dear user, what's ur name?");
    let mut name: String = String::from("");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to get the name!");
    return name;
}

fn get_num(prompt: &str) -> f32 {
    let mut temp: String = String::from("");
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to get the num...");
    let temp: f32 = temp.trim().parse().expect("Invalid num!");
    return temp;
}

fn greet(name: &String) {
    println!("WELCOME, {}! Calc.io me aapka swagat hai...", name.trim());
}
