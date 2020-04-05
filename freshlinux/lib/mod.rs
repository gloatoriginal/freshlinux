use std::fs::File;
use std::process::Command;
use std::io::{Write};

pub fn main(file: &File, install: String){
    let apt_arrays = ["build-essential".to_string(), "nodejs".to_string(),
    "python3-pip".to_string(), "default-jre".to_string(), "snapd".to_string(),
    "curl".to_string(), "git".to_string()];

    let snap_arrays = ["code --classic".to_string()];

    let curl_arrays = ["rustup".to_string()];

    if apt_arrays.contains(&install) {
        write_to_apt(file, &install);
    }
    if snap_arrays.contains(&install) {
        write_to_snap(file, &install);
    }
    if curl_arrays.contains(&install){
        write_to_curl(file, &install);
    }
    
}


fn write_to_apt(mut file: &File, install: &String){
    writeln!(file, "echo \"Preparing to Install {}\"", install);
    writeln!(file, "sudo apt-get install {} -y", install);
    writeln!(file, "sleep 2\n");
    if install == "default-jre" { writeln!(file, "sudo apt-get install default-jdk -y"); }
    println!("Finished writing {} to the main.sh", install);
}

fn write_to_snap (mut file: &File, install: &String){
    writeln!(file, "echo \"Preparing to Install {}\"", install);
    writeln!(file, "sudo snap install {}", install);
    writeln!(file, "sleep 2\n");
    println!("Finished writing {} to the main.sh", install);
}

fn write_to_curl (mut file: &File, install: &String) {
    writeln!(file, "echo \"Preparing to Install {}\"", install);
    if install == "rustup"{ 
        writeln!(file, "sudo curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh");
    }
    writeln!(file, "sleep 2\n");
}


pub fn chmod_main(){
    let command = "chmod +x main.sh";
    println!("Chmod on main.sh so user can execute same installation later");
    Command::new("bash")
        .args(&["-c", command])
        .spawn()
        .expect("Failed to-> chmod +x main.sh");
}

pub fn execute(mut file: &File) {
    writeln!(file, "\nexit");
    let command = "./main.sh";
    println!("Executing [{}]", command);
    Command::new("bash")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Failed to-> start main.sh");

    println!("Thank you for using my auto installer!");
}