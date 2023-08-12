use std::env::args;
use std::process::{Command, exit, id};
use libc::{kill, getsid, SIGTERM, SIGSTOP, SIGCONT};


// pallk - Kill !ALL! process


fn main() {
    let args: Vec<String> = args().collect();

    let mut signal = SIGTERM;


    sig_n1(SIGSTOP);


    for arg in &args[1..] {
        match arg.as_str() {
            "h" | "help" | "-h" | "-help" | "--help" => help(),
            // "a" | "A" | "all" => 
            _ => {
                if let Ok(sig) = arg.parse() { signal = sig }
                else {
                    eprintln!("pallk: unknown option: {}", arg);
                    exit(1)
                }
            }
        }
    }

    let raw_output = Command::new("sh")
        .arg("-c")
        .arg("ps -A | awk {\'print $1\'}")
        .output()
        .unwrap()
        .stdout;
    let output = String::from_utf8(raw_output).unwrap();
    let vector = output.split('\n'); // [0] is "PID"
    let cur_pid = id() as i32;
    unsafe {
        for str_pid in vector {
            if let Ok(pid) = str_pid.parse() {
                if pid < 2 || pid == cur_pid || getsid(pid) == getsid(0) || getsid(pid) == 0 { continue; }
                kill(pid, signal);
            }
        }
    }

    sig_n1(SIGCONT);
}

fn sig_n1(sig: i32) {
    unsafe { kill(-1, sig); }
}

fn help() {
    println!("pallk - Kill ALL processes");
    println!("Usage: ");
    println!("    pallk h[elp] : print this help");
    println!("    pallk $ : [where $ = signal number] send signal to processes");
    exit(0)
}
