use std::io;
use std::fs::{File};
use std::io::{Write};

mod lib;

fn get_input() -> String {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed");
    x.trim().to_string()
}

fn main() {
    //create file which will be main.sh in root directory of main.
    let mut file = File::create("main.sh").unwrap();
    writeln!(file, "sudo apt-get update -y");

    println!("Thank you for using the fresh installer, I bundled up programs/runtimes I like to install on new debian machines");
    println!("Written by Gloat");
    println!("***Warning works only with apt/snap distros");
    println!("if that is not an included package manager please CTRL+C out of this");

    println!("Would you like for me to install snap and update apt right now? y for yes");
    //start of snap
    println!("Would you like Snap Installed? (required for vscode)");
    if get_input() == "y"{
        lib::main(&file, "snapd".to_string());
    }
    //end of snap
    //start of git
    println!("Would you like git Installed?");
    if get_input() == "y"{
        lib::main(&file, "git".to_string());
    }
    //end of git
    //start of curl
    println!("Would you like CURL Installed? (required for rustup)");
    if get_input() == "y"{
        lib::main(&file, "curl".to_string());
    }
    //end of curl
    //start of gcc/g++
    println!("Would you like GCC/G++ Installed?");
    if get_input() == "y"{
        lib::main(&file, "build-essential".to_string());
    }
    //end of gcc/g++
    //start of rustup
    println!("Would you like Rustup Installed? (requires curl)");
    if get_input() == "y"{
        lib::main(&file, "rustup".to_string());
    }
    //end of rustup

    //start of java
    println!("Would you like JRE and JDK Installed?");
    if get_input() == "y"{
        lib::main(&file, "default-jre".to_string());
    }
    //end of java
    //start of nodejs/npm
    println!("Would you like nodejs/npm Installed?");
    if get_input() == "y"{
        lib::main(&file, "nodejs".to_string());
    }
    //end of nodejs/npm
    //start of python/pip
    println!("Would you like python/pip Installed?");
    if get_input() == "y"{
        lib::main(&file, "python3-pip".to_string());
    }
    //end of python/pip
    //ALL SNAP INSTALLATIONS ARE HERE
    println!("Would you like VSCode installed? (requires snap)");
    if get_input() == "y" {
        lib::main(&file, "code --classic".to_string());
    }

    lib::chmod_main();
    print!("Do you want to start this installation now? if "); 
    print!("not you can always execute \"main.sh\" in this folder\n");

    if get_input() == "y"{
        lib::execute(&file);
    } else { println!("Dont worry, you can execute your installation choices later at \"main.sh\""); }
}