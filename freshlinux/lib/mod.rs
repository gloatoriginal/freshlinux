use std::fs::File;
use std::process::Command;
use std::io::prelude::*;
use std::io::{BufReader, Write};





pub fn main(file: &File, install: &String, install_type: &String){
    write_to_install(&file, &install, install_type);
}

fn write_to_install(mut file: &File, install: &String, install_type: &String){
    writeln!(file, "echo \"Preparing to Install {}\" using {}", install, install_type);
    writeln!(file, "sudo {} install {} -y", install_type, install);
    writeln!(file, "sleep 2\n");
    if install == "default-jre" { writeln!(file, "sudo apt-get install default-jdk -y"); }
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


pub fn directory_read(which_file: String) -> Vec<String> {
    let temp_file = File::open(format!("common/{}.txt", which_file))
            .expect("Unable to open Text file");
    let buff = BufReader::new(temp_file);
    let mut return_vec = vec![];
    for i in buff.lines() {
        return_vec.push(i.unwrap());
    }
    return_vec
}