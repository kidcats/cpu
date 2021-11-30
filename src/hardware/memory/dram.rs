use std::char;

use crate::hardware::ccpu::mmu::va2pa;
pub const PHYSICAL_MEMORY_SPACE: usize = 65535;
pub const PHYSICAL_MEMORY_PAGE_NUM : usize = 15;
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
    // 一条指令占据40个空间即40个bytes
    for i in 0..40 {
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
    for i in 0..40 {
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
        write_inst_dram(va2pa(pa + (0x40 * num) as u64).unwrap(), insts[num]);
    }
}

pub fn bus_read_cacheline(paddr: u64, block: &mut [u8]) {
    let ddr_base = (paddr >> 6) << 6;
    for i in 0..64 {
        unsafe {
            block[i] = PM[ddr_base as usize + i];
        }
    }
}

pub fn bus_write_cacheline(paddr: u64, block: &[u8]) {
    let ddr_base = (paddr >> 6) << 6;
    for i in 0..64{
        unsafe{
            PM[ddr_base as usize + i] = block[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hardware::ccpu::mmu::va2pa;

    use super::*;

    #[test]
    fn test_write_and_read() {
        let value: u64 = 0x5574d795faa0;
        println!("{:x}", value);
        let va_addr = 0x00007ffffffee1e8;
        // write64bits_dram(va2pa(va_addr).unwrap(), value);
        println!("{}", va2pa(0x00007ffffffee210).unwrap());
        println!("{}", va2pa(0x00007ffffffee200).unwrap());
        println!("{}", va2pa(0x00007ffffffee1f8).unwrap());
        println!("{}", va2pa(0x00007ffffffee1f0).unwrap());
        println!("{}", va2pa(0x00007ffffffee1e8).unwrap());
        println!("{}", va2pa(0x00007ffffffee1c8).unwrap());
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
    fn test_write_inst() {
        let insts_vec = vec![
            "push   %rbp                             ", // 0  0x5574d795f020
            "mov    %rsp,%rbp                        ", // 1  0x5574d795f060
            "mov    %rdi,-0x18(%rbp)                 ", // 2  0x5574d795f0a0
            "mov    %rsi,-0x20(%rbp)                 ", // 3  0x5574d795f0e0
            "mov    -0x18(%rbp),%rdx                 ", // 4  0x5574d795f120
            "mov    -0x20(%rbp),%rax                 ", // 5  0x5574d795f160
            "add    %rdx,%rax                        ", // 6  0x5574d795f1a0
            "mov    %rax,-0x8(%rbp)                  ", // 7  0x5574d795f1e0
            "mov    -0x8(%rbp),%rax                  ", // 8  0x5574d795f220
            "pop    %rbp                             ", // 9  0x5574d795f260
            "retq                                    ", // 10 0x5574d795f2a0
            "mov    %rdx,%rsi                        ", // 11 0x5574d795f2e0  <= rip
            "mov    %rax,%rdi                        ", // 12 0x5574d795f320
            "callq  $0x5574d795f020                  ", // 13 0x5574d795f360
            "mov    %rax,-0x8(%rbp)                  ", // 14 0x5574d795f3a0
        ];
        write_inst(&insts_vec, 0x5574d795f020);
        assert_eq!(
            insts_vec[1],
            read_inst_dram(va2pa(0x5574d795f060).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[2],
            read_inst_dram(va2pa(0x5574d795f0a0).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[3],
            read_inst_dram(va2pa(0x5574d795f0e0).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[4],
            read_inst_dram(va2pa(0x5574d795f120).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[5],
            read_inst_dram(va2pa(0x5574d795f160).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[6],
            read_inst_dram(va2pa(0x5574d795f1a0).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[7],
            read_inst_dram(va2pa(0x5574d795f1e0).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[8],
            read_inst_dram(va2pa(0x5574d795f220).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[9],
            read_inst_dram(va2pa(0x5574d795f260).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[10],
            read_inst_dram(va2pa(0x5574d795f2a0).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[11],
            read_inst_dram(va2pa(0x5574d795f2e0).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[12],
            read_inst_dram(va2pa(0x5574d795f320).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[13],
            read_inst_dram(va2pa(0x5574d795f360).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[14],
            read_inst_dram(va2pa(0x5574d795f3a0).unwrap()).unwrap()
        );
    }

    #[test]
    fn test_inst_part2() {
        let insts_vec = vec![
            "push   %rbp                             ", // 0  0x400000
            "mov    %rsp,%rbp                        ", // 1  0x400040
            "sub    $0x10,%rsp                       ", // 2  0x400080
            "mov    %rdi,-0x8(%rbp)                  ", // 3  0x4000c0
            "cmpq   $0x0,-0x8(%rbp)                  ", // 4  0x400100
            "jne    0x400200                         ", // 5: 0x400140 jump to 8
            "mov    $0x0,%eax                        ", // 6  0x400180
            "jmp    0x400380                         ", // 7: 0x4001c0 jump to 14
            "mov    -0x8(%rbp),%rax                  ", // 8  0x400200
            "sub    $0x1,%rax                        ", // 9  0x400240
            "mov    %rax,%rdi                        ", // 10 0x400280
            "callq  0x00400000                       ", // 11 0x4002c0
            "mov    -0x8(%rbp),%rdx                  ", // 12 0x400300
            "add    %rdx,%rax                        ", // 13 0x400340
            "leaveq                                  ", // 14 0x400380
            "retq                                    ", // 15 0x4003c0
            "mov    $0x3,%edi                        ", // 16 0x400400  rip
            "callq  0x00400000                       ", // 17 0x400440
            "mov    %rax,-0x8(%rbp)                  ", // 18 0x400480
        ];
        write_inst(&insts_vec, 0x400000);
        assert_eq!(
            insts_vec[0],
            read_inst_dram(va2pa(0x400000).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[1],
            read_inst_dram(va2pa(0x400040).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[2],
            read_inst_dram(va2pa(0x400080).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[3],
            read_inst_dram(va2pa(0x4000c0).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[4],
            read_inst_dram(va2pa(0x400100).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[5],
            read_inst_dram(va2pa(0x400140).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[6],
            read_inst_dram(va2pa(0x400180).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[7],
            read_inst_dram(va2pa(0x4001c0).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[8],
            read_inst_dram(va2pa(0x400200).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[9],
            read_inst_dram(va2pa(0x400240).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[10],
            read_inst_dram(va2pa(0x400280).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[11],
            read_inst_dram(va2pa(0x4002c0).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[12],
            read_inst_dram(va2pa(0x400300).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[13],
            read_inst_dram(va2pa(0x400340).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[14],
            read_inst_dram(va2pa(0x400380).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[15],
            read_inst_dram(va2pa(0x4003c0).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[16],
            read_inst_dram(va2pa(0x400400).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[17],
            read_inst_dram(va2pa(0x400440).unwrap()).unwrap()
        );
        assert_eq!(
            insts_vec[18],
            read_inst_dram(va2pa(0x400480).unwrap()).unwrap()
        );
    }

    #[test]
    fn te() {
        let c: u64 = 0x5574d795f020;
        for i in 0..19 {
            println!("0x{:x}", c + i * 0x40);
        }
    }


    #[test]
    fn test_cache_ddr(){
        use super::{bus_read_cacheline,bus_write_cacheline};
        let mut l : [u8;64] = [0;64];
        let mut b : [u8;64] = [0;64];
        l[0] = 0;
        l[1] = 1;
        l[2] = 2;
        l[3] = 3;
        bus_write_cacheline(0x100, &l);
        bus_read_cacheline(0x100, &mut b);
        assert_eq!(0,b[0]);
        assert_eq!(1,b[1]);
        assert_eq!(2,b[2]);
        assert_eq!(3,b[3]);

    }

}
