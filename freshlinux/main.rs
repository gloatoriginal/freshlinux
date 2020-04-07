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
    //intro
    println!("Thank you for using the fresh installer, I bundled up programs/runtimes I like to install on new debian machines");
    println!("Written by Gloat");
    println!("***Warning works only with apt/snap distros");
    println!("if that is not an included package manager please CTRL+C out of this");
    //end of intro
    //read into vectors common and literal names for installations
    let apt_common_name = lib::directory_read("apt_common".to_string());
    let apt_literal_name = lib::directory_read("apt_literal".to_string());
    let mut file = File::create("main.sh").unwrap();
    writeln!(file, "sudo apt-get update -y");
    if apt_common_name.len() <= 0 || apt_literal_name.len() <= 0 { 
        println!("one of the apt text files has nothing in it->");
        println!("apt_common.txt: {}\napt_literal.txt: {}", apt_common_name.len(), apt_literal_name.len()); 
    }
    else if apt_common_name.len() != apt_literal_name.len() { 
        println!("apt text files are not same length->");
        println!("apt_common.txt: {}\napt_literal.txt: {}", apt_common_name.len(), apt_literal_name.len()); 
    } 
    
    else { 
        for i in 0..apt_common_name.len(){
            println!("Would you like {} installed?", apt_common_name[i]);
            if get_input() == "y" { lib::main(&file, &apt_literal_name[i], &"apt-get".to_string()); }
        }
     }

     //start of snap
     let snap_common_name = lib::directory_read("snap_common".to_string());
     let snap_literal_name = lib::directory_read("snap_literal".to_string());
    println!("Starting snap settings");

    if snap_common_name.len() <= 0 || snap_literal_name.len() <= 0 { 
        println!("one of the snap text files has nothing in it->");
        println!("snap_common.txt: {}\nsnap_literal.txt: {}", snap_common_name.len(), snap_literal_name.len()); 
    }
    else if snap_common_name.len() != snap_literal_name.len() { 
        println!("snap text files are not same length->"); 
        println!("snap_common.txt: {}\nsnap_literal.txt: {}", snap_common_name.len(), snap_literal_name.len());
    } 
    else { 
        for i in 0..snap_common_name.len(){
            println!("Would you like {} installed?", snap_common_name[i]);
            if get_input() == "y" { lib::main(&file, &snap_literal_name[i], &"snap".to_string()); }
        }
     }
    

  
    lib::chmod_main();
    print!("Do you want to start this installation now? if "); 
    print!("not you can always execute \"main.sh\" in this folder\n");

    if get_input() == "y"{
        lib::execute(&file);
    } else { println!("Dont worry, you can execute your installation choices later at \"main.sh\""); }
}