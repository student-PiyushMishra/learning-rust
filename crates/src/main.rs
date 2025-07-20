use chrono::{Local, Utc};

fn main() {
    let local_time = Local::now();
    let global_time = Utc::now();
    println!("Local Time: {} \nGlobal Time: {}", local_time, global_time);
}
