
// creating a simple CLI tool/toy
    // goal to make a simple cat and grep tool


// things we need
use std::env::{ args }; // to get command line arguments
use std::fs::{ File, read_to_string }; // to read files
use std::io::{ BufReader, BufRead };
// TODO: implement active commands to listen to
    // rscat "filename.extension"
    // rsgrep "filename.extension" : this is a simple command just search for all the lines that have the search term NO REGEX YET

fn main() {

    // get the arguments
    let arguments : Vec<String> = args().collect();
    let mut state: u8 = 0;
    let mut file_path : String = String::from("");

    println!("-----------____START___-----------\n\n");


    for arg in arguments {
        // use the ref as the we are using &str to compare
        match arg.as_ref() {
            "rscat" => state = 1,
            "rsgrep" => state = 2,
            _ => {
                // check current state
                match state {
                    1 => {
                        println!("{}", read_file(&arg));
                        break;
                    },
                    2 => {
                        state = 3;
                        file_path = arg;
                    },
                    3 => {
                        println!("{}", search_file_for(&file_path, &arg));
                        break;
                    },
                    _ => state = 0
                }
            }
        }
    }

    println!("\n\n-----------____END___-----------");

}

// this is an idiots/quick/dirty way of doing rust code
fn read_file(path : &String) -> String {
    read_to_string(path).unwrap()
}

fn search_file_for(path : &String, value : &String) -> String {
    let my_file = File::open(path).unwrap();
    let mut ret : String = String::from("");

    for line in BufReader::new(my_file).lines() {
        match line {
            Ok(l) => {
                if l.contains(value) {
                    ret.push_str(&l);
                    ret.push('\n');
                }
            },
            Err(_) => break
        }
    }

    ret
}