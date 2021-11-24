use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use crate::linker::elf_struct::*;
use crate::hardware::ccpu::isa::hex_str2u;


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


fn parse_sht(str: &str) -> sh_entry {
    let line : Vec<&str> = str.split(',').collect();
    let sh_name = line[0].to_string();
    let sh_addr = hex_str2u(line[1]);
    let sh_offset = line[2].parse::<u64>().unwrap();
    let sh_size = line[3].parse::<u64>().unwrap();
    sh_entry::new(sh_name, sh_addr, sh_offset, sh_size)
}


fn parse_symtab(str:Vec<&str>) -> st_entry{
    // ["sum,STB_GLOBAL,STT_FUNC,.text,0,22"]
    let st_name = str[0].to_string();
    let st_bind = match str[1] {
        "STB_LOCAL" => StBind::StbLocal,
        "STB_GLOBAL" => StBind::StbGlobal,
        "STB_WEAK" => StBind::StbWeak,
        _default => panic!("bad stb bind ")
    };
    let st_type = match str[2] {
        "STT_FUNC" => StType::SttFunc,
        "STT_OBJECT" => StType::SttObject,
        "STT_NOTYPE" => StType::SttNotype,
        _default => panic!("bad stb type")
    };
    let st_shndx = str[3].to_string();
    let st_value = str[4].parse::<u64>().unwrap();
    let st_size = str[5].parse::<u64>().unwrap();
    st_entry::new(st_name, st_bind, st_type, st_shndx, st_value, st_size)
}

fn parse_rl(str : Vec<&str>) -> rl_entry{
    // 17,7,R_X86_64_PC32,1,-4 
    let r_row = str[0].parse::<u64>().unwrap();
    let r_col = str[1].parse::<u64>().unwrap();
    let rel_type = match str[2] {
        "R_X86_64_32" => RelType::RX86_64_32,
        "R_X86_64_PC32" => RelType::RX86_64Pc32,
        "R_X86_64_PLT_32" => RelType::RX86_64Plt32,
        _ => panic!("bad str in rel_type"),
    };
    let sym = str[3].parse::<u32>().unwrap();
    let r_addend = str[4].trim_end_matches(" ").parse::<i64>().unwrap();
    rl_entry::new(r_row, r_col, rel_type, sym, r_addend)
}

#[cfg(test)]
mod tests{
    use std::default;

    use crate::linker::{elf_struct, parse_elf::*};
    // use crate::linker::elf_struct::*;


    #[test]
    fn test_read_file(){
        let mut s = String::new();
        s.push_str("./files/sum.elf.txt");
        let mut text_strs : Vec<String> = vec![];
        read_line(s,&mut text_strs);
        assert_eq!(text_strs[0],text_strs.len().to_string());
        let line_count = text_strs[0].parse::<u64>().unwrap();
        let sht_count = text_strs[1].parse::<u64>().unwrap();
        // 即后面sht_count行都是section table
        let mut sht : Vec<sh_entry> = Vec::new();
        for i in 2..(2+sht_count){
            let sh_entry = parse_sht(text_strs[i as usize].as_str());
            sht.push(sh_entry);
        }
        // 下面处理每一个section数据
        let mut symtab : Vec<st_entry> = vec![];
        let mut rl_tab : Vec<rl_entry> = vec![];
        let mut rl_data : Vec<rl_entry> = vec![];
        for i in &sht{
            match i.sh_name.as_str() {
                ".symtab" => {
                    // add all st_entry
                    for j in 0..i.sh_size {
                        let offset = i.sh_offset as usize;
                        let symtab_str : Vec<&str>  = text_strs[offset+j as usize].split(",").collect();
                        symtab.push(parse_symtab(symtab_str));
                    }
                },
                ".rel.text" => {
                    for j in 0..i.sh_size{
                        let offset = i.sh_offset as usize;
                        let rl_str : Vec<&str> = text_strs[offset+j as usize].split(",").collect();
                        println!("{:?}",rl_str);
                        rl_tab.push(parse_rl(rl_str));
                    }
                },
             _ => println!("to do")
            }
        }
        let mut sum_elf = elf::new(
            text_strs,
            line_count,
            sht_count,
            sht,
            symtab.len(),
            symtab,
            rl_tab.len(),
            rl_tab,
            rl_data.len(),
            rl_data
        );
        println!("{:?}",sum_elf);
        
    }

    #[test]
    fn do_hello(){
        let v = "-4     ".trim_end_matches(" ");
        println!("{:?}",v.parse::<i64>().unwrap());
    }

    #[test]
    fn test_parse_sht(){
        let str = ".text,0x0,5,22";
        let sht = parse_sht(str);
        assert_eq!(".text",sht.sh_name);
        assert_eq!(0x0,sht.sh_addr);
        assert_eq!(5,sht.sh_offset);
        assert_eq!(22,sht.sh_size);
    }

}