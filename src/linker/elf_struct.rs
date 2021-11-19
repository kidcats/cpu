

struct sh_entry{
    sh_name : String,
    sh_addr : u64,
    sh_offset : u64,
    sh_size : u64
}

enum StBind {
    StbLocal,
    StbGlobal,
    StbWeak
}

enum StType {
    SttNotype,
    SttObject,
    SttFunc
}

struct st_entry{
    st_name : String,
    st_bind : StBind,
    st_type : StType,
    st_shndx : String,
    st_value : u64,
    st_size :u64
}

enum RelType {
    RX86_64_32,
    RX86_64Pc32,
    RX86_64Plt32
}

struct rl_entry{
    r_row : u64,
    r_col :u64,
    rel_type : RelType,
    sym : u32,
    r_addend : i64,
}

struct elf{
    buffer : String,
    line_count : u64,
    sht_count : u64,
    sht : sh_entry,
    symt_count : u64,
    symt : st_entry,
    rel_text_count : u64,
    reltext : rl_entry,
    rel_data_count : u64,
    rel_data : rl_entry,
}

#[cfg(test)]
mod tests{



    #[test]
    fn do_work(){
        println!("hello world");
    }
}