extern crate libc;

use libc::sigaction;

struct Daemon {

}

impl Drop for Daemon {

    fn drop(&mut self) {

        println!("!!!");
    
    }

}

fn main() {

    let daemon = Daemon{};

}

