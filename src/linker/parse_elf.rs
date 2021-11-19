use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;



// 先读取整个文件的字符信息
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_line(path : String,vec : &mut Vec<String>){
    if let Ok(lines) = read_lines(path){
        for line in lines{
            if let Ok(s) = line{
                if (!s.starts_with('/')) && (!s.is_empty()) {
                    // find // in s
                    match s.find('/') {
                        Some(index) => vec.push(s.as_str()[..index].to_string()),
                        None => vec.push(s),
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests{
    use crate::linker::parse_elf::read_line;




    #[test]
    fn test_read_file(){
        let mut s = String::new();
        s.push_str("./files/main.elf.txt");
        let mut strs : Vec<String> = vec![];
        read_line(s,&mut strs);
        assert_eq!(strs[0],strs.len().to_string());
    }
}