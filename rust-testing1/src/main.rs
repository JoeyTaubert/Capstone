use std::{io, env};
//use std::fs::File;
//use std::cmp::Ordering;
use std::process::{Command, Stdio};
//use std::path::PathBuf;
use chrono::{Local, Utc, DateTime};

fn main(){
    tshark_cap();
}

fn tshark_cap(){
    // Grab Available Interfaces
    let tshark_interfaces = Command::new("tshark")
        .arg("-D")
        .output()
        .expect("tshark failed to run. Is it installed?");
    let tshark_interfaces_str = String::from_utf8_lossy(&tshark_interfaces.stdout);
    
    // Display Available Interfaces
    println!("Interface List");
    println!("{}","-".repeat(20));
    println!("{}", &tshark_interfaces_str);

    let mut interface_choice = String::new();

    // Prompt the User to Select an Interface
    println!("Which interface would you like to use?");
    io::stdin().read_line(&mut interface_choice)
        .expect("Error: No valid interface selected");

    // Grab the current time in UTC (for filename)
    let utc_time: DateTime<Utc> = Utc::now();
    
    // Grab the current directory (for output file)
    let path = env::current_dir()
        .expect("Failed to get current directory");

    let current_dir_str = path.display().to_string();

    let file_prefix = "tshark";

    let output_file = current_dir_str.to_string() + "/captures/" + &file_prefix.to_string() + &utc_time.format("%Y-%m-%d-UTC%H-%M-%S").to_string() + ".pcap";
    // CHANGE TO ARRAY??? Build arguments for tshark capture
    // let args = format!("-i {} -c 2 -w {} -F libpcap", &interface_choice.trim(), output_file);

    // Tshark capture
    let tshark_1000p = Command::new("tshark")
        .arg("-i")
        .arg(&interface_choice.trim())
        .arg("-c")
        .arg("1000")
        .arg("-w")
        .arg(&output_file)
        .arg("-F")
        .arg("libpcap")
        .output();

    println!("{}","-".repeat(20));
    // println!("Capturing on: {}", var);
    // println!("Full command: tshark {}", &args);
    println!("Outputting at: {}", output_file);

    match tshark_1000p {
        Ok(tshark_1000p_output) => {
            if tshark_1000p_output.status.success() {
            let stdout_tshark_1000p = String::from_utf8_lossy(&tshark_1000p_output.stdout);
            println!("Command successfully executed:\n{}", stdout_tshark_1000p);
            } else {
            let stderr_tshark_1000p = String::from_utf8_lossy(&tshark_1000p_output.stderr);
            println!("Command failed:\n{}", stderr_tshark_1000p);
            }
        },
        Err(tshark_1000p_e) => println!("Error occured: {}", tshark_1000p_e),
    }
    // tshark -z io,stat
}

