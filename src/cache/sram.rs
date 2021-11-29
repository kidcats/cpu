use super::address::*;
use crate::{
    hardware::memory::dram::{bus_read_cacheline, bus_write_cacheline},
};

const BLOCK_LEN: usize = 64;
const LINE_NUM: usize = 8;
const SET_NUM: usize = 1;

#[derive(Debug, PartialEq, Eq)]
enum sram_cacheline_state {
    CacheLineInvaid, // no data
    CacheLineClean,  // clean data
    CacheLineDirty,  // changed data
}

pub struct sram_cache_line {
    state: sram_cacheline_state,
    time: i8, // timer to find LRU
    tag: usize,
    block: [u8; BLOCK_LEN],
}

impl sram_cache_line {

    pub fn new() -> Self{
        sram_cache_line { state: sram_cacheline_state::CacheLineInvaid, time: 0, tag: 0, block: [0;BLOCK_LEN] }
    }

    pub fn update_time(&mut self, new_time: i8) {
        self.time = new_time;
    }
    pub fn update_tag(&mut self, new_tag: usize) {
        self.tag = new_tag;
    }
    pub fn update_state(&mut self, new_state: sram_cacheline_state) {
        self.state = new_state;
    }
    pub fn update_block(&mut self, value: u8, index: usize) {
        self.block[index] = value;
    }
    pub fn get_block_value(&self, index: usize) -> u8 {
        self.block[index]
    }
}

pub struct sram_cache_set {
    lines: [sram_cache_line; LINE_NUM],
}

impl sram_cache_set {
    pub fn update_lines(&mut self, value: sram_cache_line, index: usize) {
        self.lines[index] = value;
    }

    pub fn new(lines : [sram_cache_line;LINE_NUM]) -> Self {
        sram_cache_set { lines: lines }
    }
}

pub struct sram_cache {
    sets: [sram_cache_set; SET_NUM],
}

impl sram_cache {
    pub fn update_lines(&mut self, value: sram_cache_set, index: usize) {
        self.sets[index] = value;
    }

    pub fn new(sets : [sram_cache_set;SET_NUM]) -> Self{

        sram_cache { sets: sets }
    }
}

impl sram_cache {
    pub fn sram_cache_read(&mut self, paaddr: u64) -> u8 {
        let address = pa_address::new(paaddr);
        let set = &mut self.sets[address.ci() as usize]; // 先用CI找组
                                                         // 然后在组里面用CT找行,同时找到LRU要替换的行和空行
        let mut victim_line_num = 1000;
        let mut invaild_line_num = 1000;
        let mut max_time = -1;
        //
        for i in 0..set.lines.len() {
            if set.lines[i].time > max_time {
                victim_line_num = i;
                max_time = set.lines[i].time;
            }
            invaild_line_num = match set.lines[i].state {
                sram_cacheline_state::CacheLineInvaid => i,
                sram_cacheline_state::CacheLineClean => 100,
                sram_cacheline_state::CacheLineDirty => 100,
            };
        }
        // 根据有效位或者说状态state判断数据是否有效
        for i in 0..set.lines.len() {
            if set.lines[i].state != sram_cacheline_state::CacheLineInvaid
                && set.lines[i].tag == address.ct()
            {
                // cache hit // 数据有效，即cache hit,利用CO（offset）获取block里面的值返回
                set.lines[i].update_time(0);
                return set.lines[i].get_block_value(address.co());
            }
        }

        // 数据无效, 即cache miss，需要从dram中取得相应的数据返回，并写入cache
        // 在写回阶段有以下几个地方要考虑
        /*1.
        1.组内有多个行，如果有空闲的行，则填入空闲的行，然后将状态变成invaid,更新LRU
        2.组内有多个行，但是没有空闲的行，则通过LRU选择一个需要丢弃的行，将丢弃的行写回dram,将新的数据写入该行 更新LRU
         */

        if invaild_line_num < LINE_NUM {
            // 没有命中但是有空闲的行可用
            bus_read_cacheline(
                address.paddr_value(),
                set.lines[invaild_line_num].block.as_mut(),
            );
            set.lines[invaild_line_num].update_time(0);
            set.lines[invaild_line_num].update_state(sram_cacheline_state::CacheLineClean);
            set.lines[invaild_line_num].update_tag(address.ct());
            return set.lines[invaild_line_num].get_block_value(address.co());
        }
        if victim_line_num < LINE_NUM {
            if set.lines[victim_line_num].state == sram_cacheline_state::CacheLineDirty {
                bus_write_cacheline(
                    address.paddr_value(),
                    set.lines[victim_line_num].block.as_mut(),
                );
            }
        }
        set.lines[victim_line_num].update_state(sram_cacheline_state::CacheLineInvaid);
        bus_read_cacheline(
            address.paddr_value(),
            set.lines[victim_line_num].block.as_mut(),
        );
        set.lines[victim_line_num].update_time(0);
        set.lines[victim_line_num].update_state(sram_cacheline_state::CacheLineClean);
        set.lines[victim_line_num].update_tag(address.ct());

        return set.lines[victim_line_num].get_block_value(address.co());
    }

    pub fn sram_cache_write(&mut self, paaddr: u64, data: u8) {
        let address = pa_address::new(paaddr);
        let set = &mut self.sets[address.ci() as usize]; // 先用CI找组
        let mut victim_line_num = 100;
        let mut invaild_line_num = 100;
        let mut max_time = -1;
        for i in 0..set.lines.len() {
            if set.lines[i].time >= max_time {
                victim_line_num = i;
                max_time = set.lines[i].time;
            }
            invaild_line_num = match set.lines[i].state {
                sram_cacheline_state::CacheLineInvaid => i,
                sram_cacheline_state::CacheLineClean => invaild_line_num,
                sram_cacheline_state::CacheLineDirty => invaild_line_num,
            };
        }
        // 根据有效位或者说状态state判断数据是否有效
        for i in 0..set.lines.len() {
            if set.lines[i].state != sram_cacheline_state::CacheLineInvaid
                && set.lines[i].tag == address.ct()
            {
                // cache hit // 数据有效，即cache hit,利用CO（offset）写入数据
                set.lines[i].update_block(data, address.co());
                set.lines[i].update_time(0);
                // 往sram中写了数据，就变成了dirty
                set.lines[i].update_state(sram_cacheline_state::CacheLineDirty);
                return;
            }
        }
        // 如果Miss那就需要试着找一个空白的写入从ddr到sram
        if invaild_line_num < LINE_NUM {
            bus_read_cacheline(
                address.paddr_value(),
                set.lines[invaild_line_num].block.as_mut(),
            );
            set.lines[invaild_line_num].update_block(data, address.co());
            set.lines[invaild_line_num].update_time(0);
            set.lines[invaild_line_num].update_tag(address.ct());
            // 往sram中新写了数据，但是和ddr保持一致
            set.lines[invaild_line_num].update_state(sram_cacheline_state::CacheLineDirty);
            return;
        }
        // 剩下的就只能替换了啊
        /* 替换只发生在两种状态，clear和dirty
        clear代表 数据和ddr里面数据一样，直接写，然后把state -> drity
        dirty代表数据已经和ddr里面不一样了，如果要替换，就只能往ddr里面写回一次同步
         */
        if victim_line_num < LINE_NUM {
            if set.lines[victim_line_num].state == sram_cacheline_state::CacheLineDirty {
                //先把当前的值写回ddr
                bus_write_cacheline(
                    address.paddr_value(),
                    set.lines[victim_line_num].block.as_mut(),
                );
                // 然后写入sram
            }
        }
        // 将新的数据读入
        bus_read_cacheline(
            address.paddr_value(),
            set.lines[victim_line_num].block.as_mut(),
        );
        // 然后写入sram
        set.lines[victim_line_num].update_block(data, victim_line_num);
        // 新写入的当然和ddr保持的不一样了
        set.lines[victim_line_num].update_state(sram_cacheline_state::CacheLineDirty);
        set.lines[victim_line_num].update_tag(address.ct());
        println!("{}",address.ct());
        set.lines[victim_line_num].update_time(0);
    }
}

#[cfg(test)]
mod tests {


    #[test]
    fn do_test() {
        println!("{}", 1 << 6)
    }

    #[test]
    fn test_sram_read_write(){
        use  super::sram_cache;
        use crate::cache::sram::{sram_cache_line,sram_cache_set};
        let line1 = sram_cache_line::new();
        let line2 = sram_cache_line::new();
        let line3 = sram_cache_line::new();
        let line4 = sram_cache_line::new();
        let line5 = sram_cache_line::new();
        let line6 = sram_cache_line::new();
        let line7 = sram_cache_line::new();
        let line8 = sram_cache_line::new();
        let lines = [line1,line2,line3,line4,line5,line6,line7,line8];
        let set1 = sram_cache_set::new(lines);
        let mut cache = sram_cache::new([set1]);
        cache.sram_cache_write(0b0001000000000010, 1);
        cache.sram_cache_write(0b0010000000000011, 2);
        cache.sram_cache_write(0b0011000000000001, 3);
        cache.sram_cache_write(0b0100000000000011, 4);
        cache.sram_cache_write(0b0101000000000011, 5);
        cache.sram_cache_write(0b0110000000000011, 6);
        cache.sram_cache_write(0b0111000000000011, 7);
        cache.sram_cache_write(0b1000000000000011, 8);
        cache.sram_cache_write(0b0001000000000011, 9);
        assert_eq!(cache.sram_cache_read(0b0001000000000010),1);
        assert_eq!(cache.sram_cache_read(0b0010000000000011),2);
        assert_eq!(cache.sram_cache_read(0b0011000000000001),3);
        assert_eq!(cache.sram_cache_read(0b0100000000000011),4);
        assert_eq!(cache.sram_cache_read(0b0101000000000011),5);
        assert_eq!(cache.sram_cache_read(0b0110000000000011),6);
        assert_eq!(cache.sram_cache_read(0b0111000000000011),7);
        assert_eq!(cache.sram_cache_read(0b1000000000000011),8);
        assert_eq!(cache.sram_cache_read(0b0001000000000011),9);
        assert_eq!(cache.sram_cache_read(0b0001000000000010),1);
    }


}
