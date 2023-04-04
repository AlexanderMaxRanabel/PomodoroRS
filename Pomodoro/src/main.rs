use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Enter how long interval will be");
    let mut raw_interval_m: String = String::new();
    io::stdin().read_line(&mut raw_interval_m).expect("Failed to read input");
    let raw_interval_m:&str = raw_interval_m.trim();
    let interval_nx: u64 = raw_interval_m.trim().parse().expect("Input was not an integer");
    let interval: u64 = interval_nx * 60;

    println!("Enter how long break will be");
    let mut raw_break_m: String = String::new();
    io::stdin().read_line(&mut raw_break_m).expect("Failed to read input");
    let raw_break_m= raw_break_m.trim();
    let breakm_nx: u64 = raw_break_m.trim().parse().expect("Input was not an integer");
    let breakm: u64 = breakm_nx * 60;

    let mut checker: i32 = 0;

    loop {
        println!("Session has been started");
        thread::sleep(Duration::from_secs(interval));
        println!("Okay its time for a break");
        checker += 1;
        thread::sleep(Duration::from_secs(breakm));

        if checker == 4 {
            let super_break: u64 = breakm * 2;
            println!("Time for a bigger break!");
            thread::sleep(Duration::from_secs(super_break));
            checker = 0;
        }
    }
}
