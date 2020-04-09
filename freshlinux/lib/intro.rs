pub fn intro() {
    //intro
    println!("Thank you for using the fresh installer, I bundled up programs/runtimes I like to install on new debian machines");
    println!("Written by Gloat");
    println!("***Warning works only with apt/snap distros");
    println!("if that is not an included package manager please CTRL+C out of this");
    //end of intro
}

pub fn check_vectors(common_name: &Vec<String>, literal_name: &Vec<String>) -> bool {
    if common_name.len() <= 0 || literal_name.len() <= 0 { 
        println!("one of the text files has nothing in it->");
        println!("common.txt: {}\napt_literal.txt: {}", common_name.len(), literal_name.len()); 
        true
    }
    else if common_name.len() != literal_name.len() { 
        println!("apt text files are not same length->");
        println!("common.txt: {}\nliteral.txt: {}", common_name.len(), literal_name.len());
        true 
    }else{ false }
}