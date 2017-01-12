use std::env;
use std::process::*;
use std::path::Path;

fn main() {

    let mut args = env::args();
    let result = args.find(|x| x == "-d" || x == "--daemon");

    match result {
        Some(_) => daemonise(),
        None => {}
    }

    println!("Working!");

    loop { }

}

fn daemonise() {

    let file = env::current_exe().unwrap();
    let file = file.to_str().unwrap();
    let root = Path::new("/");

    Command::new(file)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .current_dir(&root)
        .spawn()
        .expect("Failed spawn process");

    exit(1);

}
