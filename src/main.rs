extern crate libc;

use std::mem;
use std::default::Default;
use std::io::Write;
use std::fs::File;
use std::fs::remove_file;
use std::env;
use std::process::*;
use std::path::Path;

struct Daemonize {
    pid_file: String,
    cwd: String,
}

impl Default for Daemonize {

    fn default() -> Self {

        Self {
            pid_file: "/tmp/programm.pid".to_string(),
            cwd: "/".to_string()
        }

    }

}

impl Daemonize {

    fn create_pid_file(&self) {

        unsafe {

            let pid = libc::getpid();
            let mut file = File::create(&self.pid_file).unwrap();
            file.write_all(pid.to_string().as_bytes()).unwrap();

        }

    }

    fn remove_pid_file(&self) {

        remove_file(&self.pid_file).unwrap();

    }

    fn sig_handler(&self) {

        self.remove_pid_file();
        exit(0);

    }

    fn start(&self) {

        let file = env::current_exe().unwrap();
        let file = file.to_str().unwrap();
        let cwd = Path::new(&self.cwd);

        Command::new(file)
            .args(env::args_os())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .current_dir(&cwd)
            .spawn()
            .expect("Failed spawn process");

        exit(1);

    }

}

fn sig_handler() {

    println!("123");
    exit(0);

}

fn main() {

    println!("{:?}", env::args_os());

    // let mut daemonize = Daemonize::default();
    // daemonize.start();

    // unsafe {

    //     let mut new_action: libc::sigaction = mem::uninitialized();
    //     let mut old_action: libc::sigaction = mem::uninitialized();

    //     new_action.sa_sigaction = sig_handler as usize;

    //     libc::sigaction(
    //         libc::SIGINT,
    //         &new_action as *const libc::sigaction,
    //         &mut old_action as *mut libc::sigaction
    //     );

    //     libc::sigaction(
    //         libc::SIGTERM,
    //         &new_action as *const libc::sigaction,
    //         &mut old_action as *mut libc::sigaction
    //     );

    // }

    // loop {}

}

