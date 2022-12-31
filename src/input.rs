use std::{fs::File, io::Read};

pub fn read_input(file_name: &str) -> String {
    let mut file = File::open(file_name).unwrap_or_else(|x| panic!("{}", x));
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap(); 
    return buf;
}