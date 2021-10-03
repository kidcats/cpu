pub const PHYSICAL_MEMORY_SPACE: usize = 1000;
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
        result += (pm[index + 8] as u64) << 8;
        result += (pm[index + 16] as u64) << 16;
        result += (pm[index + 24] as u64) << 24;
        result += (pm[index + 32] as u64) << 32;
        result += (pm[index + 40] as u64) << 40;
        result += (pm[index + 48] as u64) << 48;
        result += (pm[index + 56] as u64) << 56;
    }
    return Some(result);
}

pub fn write64bits_dram(pa_addr: u64, value: u64) {
    let index = pa_addr as usize;
    unsafe {
        pm[index + 0] = (value >> 0) as u8; 
        pm[index + 8] = (value >> 8) as u8; 
        pm[index + 16] = (value >> 16) as u8; 
        pm[index + 24] = (value >> 24) as u8; 
        pm[index + 32] = (value >> 32) as u8; 
        pm[index + 40] = (value >> 40) as u8; 
        pm[index + 48] = (value >> 48) as u8; 
        pm[index + 56] = (value >> 56) as u8; 
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_write_and_read(){
        let value:u64 = 0x1234abcdff11ff11;
        println!("{:x}",value >> 8);
        println!("{:x}",value >> 16);
        let pa_addr = 100;
        write64bits_dram(pa_addr, value);
        let read_value = read64bits_dram(pa_addr).unwrap();
        assert_eq!(read_value,value);
    }
}