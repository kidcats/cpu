#[derive(Debug)]
pub struct sh_entry {
    pub sh_name: String,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
}
impl sh_entry {
    pub fn new(sh_name: String, sh_addr: u64, sh_offset: u64, sh_size: u64) -> Self {
        sh_entry {
            sh_name: sh_name,
            sh_addr: sh_addr,
            sh_offset: sh_offset,
            sh_size: sh_size,
        }
    }
}

#[derive(Debug)]
pub enum StBind {
    StbLocal,
    StbGlobal,
    StbWeak,
}

#[derive(Debug)]
pub enum StType {
    SttNotype,
    SttObject,
    SttFunc,
}

#[derive(Debug)]
pub struct st_entry {
    pub st_name: String,
    pub st_bind: StBind,
    pub st_type: StType,
    pub st_shndx: String,
    pub st_value: u64,
    pub st_size: u64,
}

impl st_entry {
    pub fn new(
        st_name: String,
        st_bind: StBind,
        st_type: StType,
        st_shndx: String,
        st_value: u64,
        st_size: u64,
    ) -> Self {
        st_entry {
            st_name: st_name,
            st_bind: st_bind,
            st_type: st_type,
            st_shndx: st_shndx,
            st_value: st_value,
            st_size: st_size,
        }
    }
}

#[derive(Debug)]
pub enum RelType {
    RX86_64_32,
    RX86_64Pc32,
    RX86_64Plt32,
}

#[derive(Debug)]
pub struct rl_entry {
    pub r_row: u64,
    pub r_col: u64,
    pub rel_type: RelType,
    pub sym: u32,
    pub r_addend: i64,
}

impl rl_entry {
    pub fn new(r_row: u64, r_col: u64, rel_type: RelType, sym: u32, r_addend: i64) -> Self {
        rl_entry {
            r_row: r_row,
            r_col: r_col,
            rel_type: rel_type,
            sym: sym,
            r_addend: r_addend,
        }
    }
}

#[derive(Debug)]
pub struct elf {
    pub buffer: Vec<String>,
    pub line_count: u64,
    pub sht_count: u64,
    pub sht: Vec<sh_entry>,
    pub symt_count: usize,
    pub symt: Vec<st_entry>,
    pub rel_text_count: usize,
    pub reltext: Vec<rl_entry>,
    pub rel_data_count: usize,
    pub rel_data: Vec<rl_entry>,
}

impl elf {
    pub fn new(buffer:Vec<String>,line_count:u64,
        sht_count: u64,sht: Vec<sh_entry>,symt_count: usize,
        symt: Vec<st_entry>,rel_text_count: usize,reltext: Vec<rl_entry>,
        rel_data_count: usize,rel_data: Vec<rl_entry>) -> Self{
        elf { buffer: buffer, line_count: line_count, sht_count: sht_count, sht: sht, symt_count: symt_count, symt: symt, rel_text_count: rel_text_count, reltext: reltext, rel_data_count: rel_data_count, rel_data: rel_data }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn do_work() {
        println!("hello world");
    }
}
