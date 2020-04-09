use std::fs::{File};
use std::io::{Write};
use std::io;

mod lib;

fn get_input() -> String {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed");
    x.trim().to_string()
}

fn main() {
    lib::app_intro();
    //read into vectors common and literal names for installations
    let mut common_name = lib::directory_read("apt_common".to_string());
    let mut literal_name = lib::directory_read("apt_literal".to_string());
    let mut file = File::create("main.sh").unwrap();
    writeln!(file, "sudo apt-get update -y");
    if lib::name_check_vectors(&common_name, &literal_name){
        println!("Something is wrong with your apt lists.")
    } else { 
        for i in 0..common_name.len(){
            println!("Would you like {} installed?", common_name[i]);
            if get_input() == "y" { lib::main(&file, &literal_name[i], &"apt-get".to_string()); }
        }
     }
     //start of snap
     common_name = lib::directory_read("snap_common".to_string());
     literal_name = lib::directory_read("snap_literal".to_string());
    println!("Starting snap settings");
    if lib::name_check_vectors(&common_name, &literal_name){ println!("Something is wrong with your snap lists.") } 
    else { 
        for i in 0..common_name.len(){
            println!("Would you like {} installed?", common_name[i]);
            if get_input() == "y" { lib::main(&file, &literal_name[i], &"snap".to_string()); }
        }
     }
    lib::chmod_main();
    print!("Do you want to start this installation now? if "); 
    print!("not you can always execute \"main.sh\" in this folder\n");

    if get_input() == "y"{
        lib::execute(&file);
    } else { println!("Dont worry, you can execute your installation choices later at \"main.sh\""); }
}