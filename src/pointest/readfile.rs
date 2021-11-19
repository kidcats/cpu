use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self,BufRead};
use std::path::Path;

fn do_path() {
    let path = Path::new(".");
    let _display = path.display();
    let new_path = path.join("a").join("b");
    match new_path.to_str() {
        Some(s) => println!("new path is {}", s),
        None => todo!(),
    }
}

fn do_read() {
    let path = Path::new("/home/kid/document/rust_learn/cpu/files/main.elf.txt");
    let _display = path.display();
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("couldn't open {} : {}", _display, err.to_string()),
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => println!("{} contains : \n{}", _display, s),
        Err(err) => panic!(("couldn't read {} : {}", _display, err.to_string())),
    }
}

fn do_write() {
    let _path = Path::new("/home/kid/document/rust_learn/cpu/files/main.elf.txt");
    let write_path = Path::new("./files/hello.txt");
    let _r_display = _path.display();
    let w_dispaly = write_path.display();
    let mut file = match File::create(&write_path) {
        Ok(file) => file,
        Err(err) => panic!(("couldn't create {} : {}", _r_display, err.to_string())),
    };
    match file.write_all("HELLO".as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", w_dispaly, why.to_string()),
        Ok(_) => println!("successfully wrote to {}", w_dispaly),
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_line(){
    if let Ok(lines) = read_lines("./hello.txt"){
        for line in lines{
            if let Ok(s) = line{
                println!("{}",s);
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn do_work() {
        println!("hello world");
        read_line();
    }
}
