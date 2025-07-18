fn main () {
    println!("{}",fib(6));
}

fn fib(num: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;
    if num == 0 {return 0}
    if num == 1 {return 1}
    for _i in 2..=num {
        let temp = second;
        second = first + second;
        first = temp;
    }
    return second;
}
