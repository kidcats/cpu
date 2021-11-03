pub const PHYSICAL_MEMORY_SPACE: usize = 10000;
pub static mut pm: [u8; PHYSICAL_MEMORY_SPACE] = [0; PHYSICAL_MEMORY_SPACE];

/**
 * 通过实际的物理地址返回地址上的值 小端存储
 */
pub fn read64bits_dram(pa_addr: u64) -> Option<u64> {
    // if pa_addr {

    // }
    let index = pa_addr as usize;
    let mut result: u64 = 0;
    unsafe {
        result += (pm[index + 0] as u64) << 0;
        result += (pm[index + 1] as u64) << 8;
        result += (pm[index + 2] as u64) << 16;
        result += (pm[index + 3] as u64) << 24;
        result += (pm[index + 4] as u64) << 32;
        result += (pm[index + 5] as u64) << 40;
        result += (pm[index + 6] as u64) << 48;
        result += (pm[index + 7] as u64) << 56;
    }
    return Some(result);
}

pub fn write64bits_dram(pa_addr: u64, value: u64) {
    let index = pa_addr as usize;
    unsafe {
        pm[index + 0] = (value >> 0) as u8;
        pm[index + 1] = (value >> 8) as u8;
        pm[index + 2] = (value >> 16) as u8;
        pm[index + 3] = (value >> 24) as u8;
        pm[index + 4] = (value >> 32) as u8;
        pm[index + 5] = (value >> 40) as u8;
        pm[index + 6] = (value >> 48) as u8;
        pm[index + 7] = (value >> 56) as u8;
    }
}

pub fn write_inst_dram(pa_addr: u64, str: &str) {
    // 一条指令占据c0个空间即24个bytes
    for i in 0..24 {
        if i < str.len() {
            let char = str.as_bytes()[i];
            // println!("{}",char);
            unsafe {
                pm[pa_addr as usize + i] = char;
            }
        } else {
            unsafe {
                pm[pa_addr as usize + i] = (' ' as char) as u8;
            }
        }
    }
}

pub fn read_inst_dram(pa_addr: u64) -> Option<String> {
    let mut s = String::new();
    for i in 0..24 {
        unsafe {
            let char = pm[pa_addr as usize + i] as char;
            s.push(char as char);
        }
    }
    Some(s)
}

#[cfg(test)]
mod tests {
    use crate::hardware::ccpu::mmu::va2pa;

    use super::*;

    #[test]
    fn test_write_and_read() {
        let value: u64 = 0x5574d795faa0;
        println!("{:x}", value );
        let va_addr = 0x00007ffffffee1e8;
        // write64bits_dram(va2pa(va_addr).unwrap(), value);
        println!("{}",va2pa(0x00007ffffffee210).unwrap());
        println!("{}",va2pa(0x00007ffffffee200).unwrap());
        println!("{}",va2pa(0x00007ffffffee1f8).unwrap());
        println!("{}",va2pa(0x00007ffffffee1f0).unwrap());
        println!("{}",va2pa(0x00007ffffffee1e8).unwrap());
        println!("{}",va2pa(0x00007ffffffee1c8).unwrap());
        write64bits_dram(va2pa(0x00007ffffffee210).unwrap(), 0x0000000008000660); // rbp
        write64bits_dram(va2pa(0x00007ffffffee200).unwrap(), 0xabcd);
        write64bits_dram(va2pa(0x00007ffffffee1c0).unwrap(), 0xabcd);
        write64bits_dram(va2pa(0x00007ffffffee1c8).unwrap(), 0x12340000);
        write64bits_dram(va2pa(0x00007ffffffee1e0).unwrap(), 0x7ffffffee210);
        write64bits_dram(va2pa(0x00007ffffffee1e8).unwrap(), 0x5574d795faa0);
        write64bits_dram(va2pa(0x00007ffffffee1f0).unwrap(), 0x8000660);
        write64bits_dram(va2pa(0x00007ffffffee1f8).unwrap(), 0x12340000);
        let read_value = read64bits_dram(va2pa(va_addr).unwrap()).unwrap();
        assert_eq!(read_value, value);
    }

    #[test]
    fn test_write_inst(){
        let isnt = "callq  0x5574d795f020";
        write_inst_dram(va2pa(0x5574d795f020).unwrap(), isnt);
        let result = read_inst_dram(va2pa(0x5574d795f020).unwrap()).unwrap();
        assert_eq!(isnt,result);
    }
}
