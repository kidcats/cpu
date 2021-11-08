use std::{char};

use crate::hardware::ccpu::mmu::va2pa;
pub const PHYSICAL_MEMORY_SPACE: usize = 19200;
pub static mut PM: [u8; PHYSICAL_MEMORY_SPACE] = [0; PHYSICAL_MEMORY_SPACE];

/**
 * 通过实际的物理地址返回地址上的值 小端存储
 */
pub fn read64bits_dram(pa_addr: u64) -> Option<u64> {
    // if pa_addr {

    // }
    let index = pa_addr as usize;
    let mut result: u64 = 0;
    unsafe {
        result += (PM[index + 0] as u64) << 0;
        result += (PM[index + 1] as u64) << 8;
        result += (PM[index + 2] as u64) << 16;
        result += (PM[index + 3] as u64) << 24;
        result += (PM[index + 4] as u64) << 32;
        result += (PM[index + 5] as u64) << 40;
        result += (PM[index + 6] as u64) << 48;
        result += (PM[index + 7] as u64) << 56;
    }
    return Some(result);
}

pub fn write64bits_dram(pa_addr: u64, value: u64) {
    let index = pa_addr as usize;
    unsafe {
        PM[index + 0] = (value >> 0) as u8;
        PM[index + 1] = (value >> 8) as u8;
        PM[index + 2] = (value >> 16) as u8;
        PM[index + 3] = (value >> 24) as u8;
        PM[index + 4] = (value >> 32) as u8;
        PM[index + 5] = (value >> 40) as u8;
        PM[index + 6] = (value >> 48) as u8;
        PM[index + 7] = (value >> 56) as u8;
    }
}

pub fn write_inst_dram(pa_addr: u64, str: &str) {
    // 一条指令占据c0个空间即24个bytes
    for i in 0..24 {
        if i < str.len() {
            let char = str.as_bytes()[i];
            // println!("{}",char);
            unsafe {
                PM[pa_addr as usize + i] = char;
            }
        } else {
            unsafe {
                PM[pa_addr as usize + i] = (' ' as char) as u8;
            }
        }
    }
}

pub fn read_inst_dram(pa_addr: u64) -> Option<String> {
    let mut s = String::new();
    for i in 0..24 {
        unsafe {
            let char = PM[pa_addr as usize + i] as char;
            s.push(char as char);
        }
    }
    Some(s.trim_end_matches(char::from(0)).to_string())
}

// 将指令写入硬盘
pub fn write_inst(insts: &Vec<&str>, pa: u64) {
    for num in 0..insts.len() {
        write_inst_dram(va2pa(pa + (0xc0 * num) as u64).unwrap(), insts[num]);
    }
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
        let insts_vec = vec![
            "push   %rbp             ",       // 0  0x5574d795f020
            "mov    %rsp,%rbp        ",       // 1  0x5574d795f0e0
            "mov    %rdi,-0x18(%rbp) ",       // 2  0x5574d795f1a0
            "mov    %rsi,-0x20(%rbp) ",       // 3  0x5574D795F260
            "mov    -0x18(%rbp),%rdx ",       // 4  0x5574D795F320
            "mov    -0x20(%rbp),%rax ",       // 5  0x5574d795f3e0
            "add    %rdx,%rax        ",       // 6  0x5574D795F4A0
            "mov    %rax,-0x8(%rbp)  ",       // 7  0x5574D795F560
            "mov    -0x8(%rbp),%rax  ",       // 8  0x5574D795F620
            "pop    %rbp             ",       // 9  0x5574D795F6E0
            "retq                    ",       // 10 0x5574D795F7A0
            "mov    %rdx,%rsi        ",       // 11 0x5574d795f860  <= rip
            "mov    %rax,%rdi        ",       // 12 0x5574d795f920
            "callq  $0x5574d795f020  ",       // 13 0x5574d795f9e0
            "mov    %rax,-0x8(%rbp)  ",       // 14 0x5574d795faa0
        ];
        write_inst(&insts_vec, 0x5574d795f020);
        assert_eq!(insts_vec[0],read_inst_dram(va2pa(0x5574d795f020).unwrap()).unwrap());
        assert_eq!(insts_vec[1],read_inst_dram(va2pa(0x5574d795f0e0).unwrap()).unwrap());
        assert_eq!(insts_vec[2],read_inst_dram(va2pa(0x5574d795f1a0).unwrap()).unwrap());
        assert_eq!(insts_vec[3],read_inst_dram(va2pa(0x5574D795F260).unwrap()).unwrap());
        assert_eq!(insts_vec[4],read_inst_dram(va2pa(0x5574D795F320).unwrap()).unwrap());
        assert_eq!(insts_vec[5],read_inst_dram(va2pa(0x5574d795f3e0).unwrap()).unwrap());
        assert_eq!(insts_vec[6],read_inst_dram(va2pa(0x5574D795F4A0).unwrap()).unwrap());
        assert_eq!(insts_vec[7],read_inst_dram(va2pa(0x5574D795F560).unwrap()).unwrap());
        assert_eq!(insts_vec[8],read_inst_dram(va2pa(0x5574D795F620).unwrap()).unwrap());
        assert_eq!(insts_vec[9],read_inst_dram(va2pa(0x5574D795F6E0).unwrap()).unwrap());
        assert_eq!(insts_vec[10],read_inst_dram(va2pa(0x5574D795F7A0).unwrap()).unwrap());
        assert_eq!(insts_vec[11],read_inst_dram(va2pa(0x5574d795f860).unwrap()).unwrap());
        assert_eq!(insts_vec[12],read_inst_dram(va2pa(0x5574d795f920).unwrap()).unwrap());
        assert_eq!(insts_vec[13],read_inst_dram(va2pa(0x5574d795f9e0).unwrap()).unwrap());
        assert_eq!(insts_vec[14],read_inst_dram(va2pa(0x5574d795faa0).unwrap()).unwrap());
    }
}
