use super::address::*;

#[derive(Debug)]
enum sram_cacheline_state {
    CACHE_LINE_INVAID,
    CACHE_LINE_CLEAN,
    CACHE_LINE_DIRTY,
}

pub struct sram_cache_line {
    state: sram_cacheline_state,
    time: i8, // timer to find LRU
    tag: u64,
    block: [u8; 64],
}

pub struct sram_cache_set {
    lines: [sram_cache_line; 64],
}

pub struct sram_cache {
    sets: [sram_cache_set; 64],
}

impl sram_cache {
    pub fn sram_cache_read(&self, paaddr: u64) -> u8 {
        let address = pa_address::new(paaddr);
        let set = self.sets[address.ci()]; // 先找组
        // 然后在组里面找行
        todo!()
    }

    pub fn sram_cache_write(&self, paaddr: u64, data: u8) {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn do_test() {
        println!("{}", 1 << 6)
    }
}
