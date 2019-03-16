
// let's create a proper cli tool

// let's start with a proper cli parser first
use clap::{Arg, App};

fn main() {
    let matches = App::new("rscat")
        .version("0.1.0")
        .author("Swarnim Arun <swarnimarun11@gmail.com>")
        .about("cat command clone in rust")
        .arg(Arg::with_name("file_path")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("path to file that is to be read"))
        .get_matches();
    
    let path = matches.value_of("file_path").unwrap();
    println!("{}", std::fs::read_to_string(path).unwrap());
}
