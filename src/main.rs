use std::env;
use std::process::Command;
use std::process::exit;

fn main() {

    let mut args = env::args();
    let result = args.find(| x | x == "-d");

    match result {
        Some(_) => daemonise(),
        None => {},
    }

    loop {
        println!("!!!");
    }

}

fn daemonise() {

    let file = env::current_exe().unwrap();
    let file = file.to_str().unwrap();

    let child = Command::new(file)
        .spawn()
        .expect("Failed spawn process");

    exit(1);

}
