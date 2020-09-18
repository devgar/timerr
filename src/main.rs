use std::env;
use std::{thread::sleep, time::Duration};

fn parse(arg: String) -> Option<u32> {
    Some(arg.parse::<u32>().expect("Argument type must be an unsigned integer"))
}

fn main() {
    let time: u32 = env::args().nth(1).and_then(parse).unwrap_or(6);
    let err : u32 = env::args().nth(2).and_then(parse).unwrap_or(1);
    sleep(Duration::from_secs(time as u64));
    std::process::exit(err as i32);
}
