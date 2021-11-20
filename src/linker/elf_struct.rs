#[derive(Debug)]
pub struct sh_entry {
    pub sh_name: String,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
}
impl sh_entry {
    pub fn new(
        sh_name: String, sh_addr: u64, sh_offset: u64, sh_size: u64
    ) -> Self {
        sh_entry { 
            sh_name: sh_name, sh_addr: sh_addr, sh_offset: sh_offset, sh_size: sh_size 
        }
    }
}

pub enum StBind {
    StbLocal,
    StbGlobal,
    StbWeak,
}

pub enum StType {
    SttNotype,
    SttObject,
    SttFunc,
}

pub struct st_entry {
    pub st_name: String,
    pub st_bind: StBind,
    pub st_type: StType,
    pub st_shndx: String,
    pub st_value: u64,
    pub st_size: u64,
}

impl st_entry {
    fn new(
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

pub enum RelType {
    RX86_64_32,
    RX86_64Pc32,
    RX86_64Plt32,
}

pub struct rl_entry {
    pub r_row: u64,
    pub r_col: u64,
    pub rel_type: RelType,
    pub sym: u32,
    pub r_addend: i64,
}

pub struct elf {
    pub buffer: String,
    pub line_count: u64,
    pub sht_count: u64,
    pub sht: sh_entry,
    pub symt_count: u64,
    pub symt: st_entry,
    pub rel_text_count: u64,
    pub reltext: rl_entry,
    pub rel_data_count: u64,
    pub rel_data: rl_entry,
}

#[cfg(test)]
mod tests {

    #[test]
    fn do_work() {
        println!("hello world");
    }
}
