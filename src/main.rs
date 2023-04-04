use dotext::{self, MsDoc};
use std::fs::File;
use std::io::Read;
use std::ops::Not;

fn main() {
    // Declare all the variables
    let args: Vec<String> = std::env::args().collect();
    let mut file_docx;
    let mut file;
    let mut data = String::new();

    // Check if there is the right number of arguments
    if (args.len() == 2).not() {
        panic!("Wrong number of arguments")
    }

    // Handle .docx files
    if &args[1].ends_with(".docx") == &true {
        file_docx = dotext::Docx::open(&args[1]).unwrap();
        file_docx
            .read_to_string(&mut data)
            .expect("Error while reading file");
    } else {
        file = File::open(&args[1]).expect("File not found");
        file
            .read_to_string(&mut data)
            .expect("Error while reading file");
    }

    //Print the output
    println!("{}", data);
}
