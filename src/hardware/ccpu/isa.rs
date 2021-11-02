#![allow(unused)]

use super::mmu::va2pa;
use crate::hardware::memory::dram::*;
use core::fmt;
use std::{collections::btree_set::Union, default, fmt::Result, mem::size_of, rc::Rc, string};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct RAX_Inner {
    al: u8, // 要低地址在前，高地址在后
    ah: u8,
}
#[repr(C)]
union RAX_REG {
    rax: u64,
    eax: u32,
    ax: u16,
    inner: RAX_Inner,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RBX_Inner {
    bl: u8,
    bh: u8,
}
#[repr(C)]
union RBX_REG {
    inner: RBX_Inner,
    bx: u16,
    ebx: u32,
    rbx: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RCX_Inner {
    cl: u8,
    ch: u8,
}
#[repr(C)]
union RCX_REG {
    inner: RCX_Inner,
    cx: u16,
    ecx: u32,
    rcx: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RDX_Inner {
    dl: u8,
    dh: u8,
}

#[repr(C)]
union RDX_REG {
    inner: RDX_Inner,
    dx: u16,
    edx: u32,
    rdx: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RSI_Inner {
    sil: u8,
    sih: u8,
}
#[repr(C)]
union RSI_REG {
    inner: RSI_Inner,
    si: u16,
    esi: u32,
    rsi: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RDI_Inner {
    dil: u8,
    dih: u8,
}
#[repr(C)]
union RDI_REG {
    inner: RDI_Inner,
    di: u16,
    edi: u32,
    rdi: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RBP_Inner {
    bpl: u8,
    bph: u8,
}
#[repr(C)]
union RBP_REG {
    inner: RBP_Inner,
    bp: u16,
    ebp: u32,
    rbp: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RSP_Inner {
    spl: u8,
    sph: u8,
}
#[repr(C)]
union RSP_REG {
    inner: RSP_Inner,
    sp: u16,
    esp: u32,
    rsp: u64,
}

#[repr(C)]
union R8_REG {
    r8b: u8,
    r8w: u16,
    r8d: u32,
    r8: u64,
}

#[repr(C)]
union R9_REG {
    r9b: u8,
    r9w: u16,
    r9d: u32,
    r9: u64,
}
#[repr(C)]
union R10_REG {
    r10b: u8,
    r10w: u16,
    r10d: u32,
    r10: u64,
}
#[repr(C)]
union R11_REG {
    r11b: u8,
    r11w: u16,
    r11d: u32,
    r11: u64,
}
#[repr(C)]
union R12_REG {
    r12b: u8,
    r12w: u16,
    r12d: u32,
    r12: u64,
}
#[repr(C)]
union R13_REG {
    r13b: u8,
    r13w: u16,
    r13d: u32,
    r13: u64,
}
#[repr(C)]
union R14_REG {
    r14b: u8,
    r14w: u16,
    r14d: u32,
    r14: u64,
}
#[repr(C)]
union R15_REG {
    r15b: u8,
    r15w: u16,
    r15d: u32,
    r15: u64,
}

#[repr(C)]
union RIP_REG {
    rip: u64,
    eip: u32,
}

#[derive(Debug, Clone)]
enum OD {
    EMPTY,
    IMM(u64),
    REG64(u64, String),
    #[allow(non_camel_case_types)]
    M_IMM(u64),
    #[allow(non_camel_case_types)]
    M_REG(u64), // #[allow(non_camel_case_types)]
                // M_IMM_REG(u64),
                // #[allow(non_camel_case_types)]
                // M_REG_REG(u64),
                // #[allow(non_camel_case_types)]
                // M_IMM_REG_REG(u64),
                // #[allow(non_camel_case_types)]
                // M_REG_S(u64),
                // #[allow(non_camel_case_types)]
                // M_IMM_REG_S(u64),
                // #[allow(non_camel_case_types)]
                // M_REG_REG_S(u64),
                // #[allow(non_camel_case_types)]
                // M_IMM_REG_REG_S(u64)
}

// 需要的指令类型
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
enum INST_TYPE {
    MOV,
    PUSH,
    POP,
    LEAVE,
    CALL,
    RET,
    ADD,
    SUB,
    CMP,
    JNE,
    JMP,
}

#[derive(Debug)]
struct Inst {
    inst_type: INST_TYPE,
    src: OD, // mov %reg %(0x188773)
    dst: OD,
}

// core 里面保存和所有的寄存器，和符号信息,符号可以先不管
struct Core {
    rax: RAX_REG,
    rbx: RBX_REG,
    rcx: RCX_REG,
    rdx: RDX_REG,
    rsi: RSI_REG,
    rdi: RDI_REG,
    rbp: RBP_REG,
    rsp: RSP_REG,
    r8: R8_REG,
    r9: R9_REG,
    r10: R10_REG,
    r11: R11_REG,
    r12: R12_REG,
    r13: R13_REG,
    r14: R14_REG,
    r15: R15_REG,
    rip: RIP_REG,
}

impl Core {
    fn new() -> Core {
        let regs = (
            RAX_REG {
                rax: 0x0000_0000_0000_0000,
            },
            RBX_REG {
                rbx: 0x0000_0000_0000_0000,
            },
            RCX_REG {
                rcx: 0x0000_0000_0000_0000,
            },
            RDX_REG {
                rdx: 0x0000_0000_0000_0000,
            },
            RSI_REG {
                rsi: 0x0000_0000_0000_0000,
            },
            RDI_REG {
                rdi: 0x0000_0000_0000_0000,
            },
            RBP_REG {
                rbp: 0x0000_0000_0000_0000,
            },
            RSP_REG {
                rsp: 0x0000_0000_0000_0000,
            },
            R8_REG {
                r8: 0x0000_0000_0000_0000,
            },
            R9_REG {
                r9: 0x0000_0000_0000_0000,
            },
            R10_REG {
                r10: 0x0000_0000_0000_0000,
            },
            R11_REG {
                r11: 0x0000_0000_0000_0000,
            },
            R12_REG {
                r12: 0x0000_0000_0000_0000,
            },
            R13_REG {
                r13: 0x0000_0000_0000_0000,
            },
            R14_REG {
                r14: 0x0000_0000_0000_0000,
            },
            R15_REG {
                r15: 0x0000_0000_0000_0000,
            },
            RIP_REG {
                rip: 0x0000_0000_0000_0000,
            },
        );
        unsafe {
            Core {
                rax: { regs.0 },
                rbx: { regs.1 },
                rcx: { regs.2 },
                rdx: { regs.3 },
                rsi: { regs.4 },
                rdi: { regs.5 },
                rbp: { regs.6 },
                rsp: { regs.7 },
                r8: { regs.8 },
                r9: { regs.9 },
                r10: { regs.10 },
                r11: { regs.11 },
                r12: { regs.12 },
                r13: { regs.13 },
                r14: { regs.14 },
                r15: { regs.15 },
                rip: { regs.16 },
            }
        }
    }
    fn update_reg(&mut self, od: &str, value: u64) {
        match od {
            "rax" => self.rax.rax = value,
            "eax" => self.rax.eax = value as u32,
            "ax" => self.rax.ax = value as u16,
            "ah" => self.rax.inner.ah = value as u8,
            "al" => self.rax.inner.al = value as u8,
            "rbx" => self.rbx.rbx = value,
            "ebx" => self.rbx.ebx = value as u32,
            "bx" => self.rbx.bx = value as u16,
            "bh" => self.rbx.inner.bh = value as u8,
            "bl" => self.rbx.inner.bl = value as u8,
            "rcx" => self.rcx.rcx = value,
            "ecx" => self.rcx.ecx = value as u32,
            "cx" => self.rcx.cx = value as u16,
            "ch" => self.rcx.inner.ch = value as u8,
            "cl" => self.rcx.inner.cl = value as u8,
            "rdx" => self.rax.rax = value,
            "edx" => self.rax.eax = value as u32,
            "dx" => self.rax.ax = value as u16,
            "dh" => self.rax.inner.ah = value as u8,
            "dl" => self.rax.inner.al = value as u8,
            "rsi" => self.rsi.rsi = value,
            "esi" => self.rsi.esi = value as u32,
            "si" => self.rsi.si = value as u16,
            "sih" => self.rsi.inner.sih = value as u8,
            "sil" => self.rsi.inner.sil = value as u8,
            "rdi" => self.rdi.rdi = value,
            "edi" => self.rdi.edi = value as u32,
            "di" => self.rdi.di = value as u16,
            "dih" => self.rdi.inner.dih = value as u8,
            "dil" => self.rdi.inner.dil = value as u8,
            "rsp" => self.rsp.rsp = value,
            "esp" => self.rsp.esp = value as u32,
            "sp" => self.rsp.sp = value as u16,
            "sph" => self.rsp.inner.sph = value as u8,
            "spl" => self.rsp.inner.spl = value as u8,
            "rbp" => self.rbp.rbp = value,
            "ebp" => self.rbp.ebp = value as u32,
            "bp" => self.rbp.bp = value as u16,
            "bph" => self.rbp.inner.bph = value as u8,
            "bpl" => self.rbp.inner.bpl = value as u8,
            "r8" => self.r8.r8 = value,
            "r8d" => self.r8.r8d = value as u32,
            "r8w" => self.r8.r8w = value as u16,
            "r8b" => self.r8.r8b = value as u8,
            "r9" => self.r9.r9 = value,
            "r9d" => self.r9.r9d = value as u32,
            "r9w" => self.r9.r9w = value as u16,
            "r9b" => self.r9.r9b = value as u8,
            "r10" => self.r10.r10 = value,
            "r10d" => self.r10.r10d = value as u32,
            "r10w" => self.r10.r10w = value as u16,
            "r10b" => self.r10.r10b = value as u8,
            "r11" => self.r11.r11 = value,
            "r11d" => self.r11.r11d = value as u32,
            "r11w" => self.r11.r11w = value as u16,
            "r11b" => self.r11.r11b = value as u8,
            "r12" => self.r12.r12 = value,
            "r12d" => self.r12.r12d = value as u32,
            "r12w" => self.r12.r12w = value as u16,
            "r12b" => self.r12.r12b = value as u8,
            "r13" => self.r13.r13 = value,
            "r13d" => self.r13.r13d = value as u32,
            "r13w" => self.r13.r13w = value as u16,
            "r13b" => self.r13.r13b = value as u8,
            "r14" => self.r14.r14 = value,
            "r14d" => self.r14.r14d = value as u32,
            "r14w" => self.r14.r14w = value as u16,
            "r14b" => self.r14.r14b = value as u8,
            "r15" => self.r15.r15 = value,
            "r15d" => self.r15.r15d = value as u32,
            "r15w" => self.r15.r15w = value as u16,
            "r15b" => self.r15.r15b = value as u8,
            "rip" => self.rip.rip = value,
            "eip" => self.rip.eip = value as u32,
            _ => panic!("bad reg name"),
        }
    }
    fn get_reg_value(&self, name: &str) -> Option<u64> {
        unsafe {
            match name {
                "%rax" => Some(self.rax.rax),
                "%rbx" => Some(self.rbx.rbx),
                "%rcx" => Some(self.rcx.rcx),
                "%rdx" => Some(self.rdx.rdx),
                "%rsi" => Some(self.rsi.rsi),
                "%rdi" => Some(self.rdi.rdi),
                "%rbp" => Some(self.rbp.rbp),
                "%rsp" => Some(self.rsp.rsp),
                "%eax" => Some(self.rax.eax as u64),
                "%ebx" => Some(self.rbx.ebx as u64),
                "%ecx" => Some(self.rcx.ecx as u64),
                "%edx" => Some(self.rdx.edx as u64),
                "%esi" => Some(self.rsi.esi as u64),
                "%edi" => Some(self.rdi.edi as u64),
                "%ebp" => Some(self.rbp.ebp as u64),
                "%esp" => Some(self.rsp.esp as u64),
                "%ax" => Some(self.rax.ax as u64),
                "%bx" => Some(self.rbx.bx as u64),
                "%cx" => Some(self.rcx.cx as u64),
                "%dx" => Some(self.rdx.dx as u64),
                "%si" => Some(self.rsi.si as u64),
                "%di" => Some(self.rdi.di as u64),
                "%bp" => Some(self.rbp.bp as u64),
                "%sp" => Some(self.rsp.sp as u64),
                "%al" => Some(self.rax.inner.al as u64),
                "%ah" => Some(self.rax.inner.ah as u64),
                "%bl" => Some(self.rbx.inner.bl as u64),
                "%bh" => Some(self.rbx.inner.bh as u64),
                "%cl" => Some(self.rcx.inner.cl as u64),
                "%ch" => Some(self.rcx.inner.ch as u64),
                "%dl" => Some(self.rdx.inner.dl as u64),
                "%dh" => Some(self.rdx.inner.dh as u64),
                "%sil" => Some(self.rsi.inner.sil as u64),
                "%sih" => Some(self.rsi.inner.sih as u64),
                "%dil" => Some(self.rdi.inner.dil as u64),
                "%dih" => Some(self.rdi.inner.dih as u64),
                "%bpl" => Some(self.rbp.inner.bpl as u64),
                "%bph" => Some(self.rbp.inner.bph as u64),
                "%spl" => Some(self.rsp.inner.spl as u64),
                "%sph" => Some(self.rsp.inner.sph as u64),
                "%r8" => Some(self.r8.r8 as u64),
                "%r9" => Some(self.r9.r9),
                "%r10" => Some(self.r10.r10),
                "%r11" => Some(self.r11.r11),
                "%r12" => Some(self.r12.r12),
                "%r13" => Some(self.r13.r13),
                "%r14" => Some(self.r14.r14),
                "%r15" => Some(self.r15.r15),
                "%r8d" => Some(self.r8.r8d as u64),
                "%r9d" => Some(self.r9.r9d as u64),
                "%r10d" => Some(self.r10.r10d as u64),
                "%r11d" => Some(self.r11.r11d as u64),
                "%r12d" => Some(self.r12.r12d as u64),
                "%r13d" => Some(self.r13.r13d as u64),
                "%r14d" => Some(self.r14.r14d as u64),
                "%r15d" => Some(self.r15.r15d as u64),
                "%r8w" => Some(self.r8.r8w as u64),
                "%r9w" => Some(self.r9.r9w as u64),
                "%r10w" => Some(self.r10.r10w as u64),
                "%r11w" => Some(self.r11.r11w as u64),
                "%r12w" => Some(self.r12.r12w as u64),
                "%r13w" => Some(self.r13.r13w as u64),
                "%r14w" => Some(self.r14.r14w as u64),
                "%r15w" => Some(self.r15.r15w as u64),
                "%r8b" => Some(self.r8.r8b as u64),
                "%r9b" => Some(self.r9.r9b as u64),
                "%r10b" => Some(self.r10.r10b as u64),
                "%r11b" => Some(self.r11.r11b as u64),
                "%r12b" => Some(self.r12.r12b as u64),
                "%r13b" => Some(self.r13.r13b as u64),
                "%r14b" => Some(self.r14.r14b as u64),
                "%r15b" => Some(self.r15.r15b as u64),
                "%rip" => Some(self.rip.rip as u64),
                "%eip" => Some(self.rip.eip as u64),
                default => Some(0 as u64),
            }
        }
    }
}

// 将输入的字符串解析成对应的inst结构
fn parse_inst_type(str: &str) -> Option<INST_TYPE> {
    match str.as_ref() {
        "mov" => Some(INST_TYPE::MOV),
        "push" => Some(INST_TYPE::PUSH),
        "pop" => Some(INST_TYPE::POP),
        "leave" => Some(INST_TYPE::LEAVE),
        "callq" => Some(INST_TYPE::CALL),
        "ret" => Some(INST_TYPE::RET),
        "add" => Some(INST_TYPE::ADD),
        "sub" => Some(INST_TYPE::SUB),
        "cmp" => Some(INST_TYPE::CMP),
        "jne" => Some(INST_TYPE::JNE),
        "jmp" => Some(INST_TYPE::JMP),
        default => None,
    }
}

// 将十六进制或者十进制的的str解析成对应的i64
fn hex_str2i(str: &str) -> i64 {
    if str.eq("") {
        return 0;
    }
    let flag = str.starts_with("-");
    let mut result: i64 = 0;
    let temp: Vec<&str> = str.split("x").collect();

    let without_prefix = temp.last().unwrap();
    if temp.len() == 1 {
        // 说明是十进制
        if flag {
            // 为负数
            result = i64::from_str_radix(without_prefix, 10).unwrap() * -1;
        } else {
            result = i64::from_str_radix(without_prefix, 10).unwrap();
        }
    } else {
        if flag {
            // 为负数
            result = i64::from_str_radix(without_prefix, 16).unwrap() * -1;
        } else {
            result = i64::from_str_radix(without_prefix, 16).unwrap();
        }
    }
    result
}

// 十六进制或者十进制字符串转u64
fn hex_str2u(str: &str) -> u64 {
    if str.eq("") {
        return 0;
    }
    let mut result: u64 = 0;
    let temp: Vec<&str> = str.split("x").collect();
    let without_prefix = temp.last().unwrap();
    // println!("{}",without_prefix);
    if temp.len() == 1 {
        // 说明是十进制
        result = u64::from_str_radix(without_prefix, 10).unwrap();
    } else {
        result = u64::from_str_radix(without_prefix, 16).unwrap();
    }
    result
}

// usize 和 isize 相加减
fn icalu(n1: i64, n2: u64) -> u64 {
    let n_temp = n1.abs() as u64;
    let result = if n1 > 0 { n2 + n_temp } else { n2 - n_temp };
    return result;
}

// 解析内存型
// -12(reg)
// -12(reg1,reg2)
// -12(reg,s)
// -12(reg1,reg2,s)
// (reg)
fn parse_mm_ist(str: &str, core: &Core) -> u64 {
    let mut state = 1;
    let mut imm = String::new();
    let mut r1 = String::new();
    let mut r2 = String::new();
    let mut scal = String::new();
    let mut imm_is_neg = false;
    let chars: Vec<char> = str.chars().collect();
    for i in chars {
        match state {
            1 => match i {
                '(' => {
                    state = 2;
                }
                '-' => {
                    imm_is_neg = true;
                    state = 3;
                }
                '0'..='9' => {
                    imm.push(i);
                    state = 4;
                }
                ' ' => {}
                _ => {
                    panic!("error in parse_mm_ist")
                }
            },
            2 => match i {
                '%' => {
                    state = 5;
                    r1.push(i);
                }
                ',' => {
                    state = 6;
                }
                ' ' => {}
                _ => {
                    panic!("error in parse_mm_ist")
                }
            },
            3 => match i {
                '0'..='9' => {
                    state = 4;
                    imm.push(i);
                }
                ' ' => {}
                _ => {
                    panic!("error in parse_mm_ist")
                }
            },
            4 => match i {
                '0'..='9' => {
                    state = 4;
                    imm.push(i);
                }
                'x' => {
                    state = 10;
                    imm.push(i);
                }
                '(' => {
                    state = 2;
                }
                ' ' => {}
                _ => {
                    panic!("error in parse_mm_ist")
                }
            },
            5 => match i {
                'a'..='z' => {
                    r1.push(i);
                    state = 7;
                }
                ' ' => {}
                _ => {
                    panic!("error in parse_mm_ist")
                }
            },
            6 => match i {
                '0'..='9' => {
                    scal.push(i);
                    state = 9;
                }
                '%' => {
                    state = 11;
                    r2.push(i);
                }
                ' ' => {}
                _ => {
                    panic!("error in parse_mm_ist")
                }
            },
            7 => match i {
                'a'..='z' => {
                    r1.push(i);
                    state = 7;
                }
                ',' => {
                    state = 6;
                }
                ')' => {
                    state = 8;
                }
                ' ' => {}
                _ => {
                    panic!("error in parse_mm_ist")
                }
            },
            8 => {
                break;
            }
            9 => match i {
                '0'..='9' => {
                    state = 9;
                    scal.push(i);
                }
                ')' => {
                    state = 8;
                }
                ' ' => {}
                _ => {
                    panic!("error in parse_mm_ist")
                }
            },
            10 => match i {
                '0'..='9' => {
                    state = 10;
                    imm.push(i);
                }
                'a'..='z' => {
                    state = 10;
                    imm.push(i);
                }
                '(' => {
                    state = 2;
                }
                ' ' => {}
                _ => {
                    panic!("error in parse_mm_ist")
                }
            },
            11 => match i {
                'a'..='z' => {
                    state = 12;
                    r2.push(i);
                }
                ' ' => {}
                _ => {
                    panic!("error in parse_mm_ist")
                }
            },
            12 => match i {
                'a'..='z' => {
                    state = 12;
                    r2.push(i);
                }
                ')' => state = 8,
                ',' => state = 6,
                ' ' => {}
                _ => {
                    panic!("error in parse_mm_ist")
                }
            },
            default => {
                // 清空
                state = 1;
                imm = String::new();
                r1 = String::new();
                r2 = String::new();
                scal = String::new();
            }
        }
    }

    println!(" {} ,{}, {}, {}", &imm, &r1, &r2, &scal);
    let scal_temp = if scal.eq("") {
        1
    } else {
        hex_str2u(scal.as_str())
    };
    let temp = core.get_reg_value(r1.as_str()).unwrap()
        + core.get_reg_value(r2.as_str()).unwrap() * scal_temp;
    println!("temp : {}", &temp);
    let temp2 = hex_str2i(imm.as_str());
    icalu(temp2, temp)
}

// 解析操作数的类型,获取对应的数值，有可能是地址，有可能是纯数值
fn parse_od_type(str: &str, core: &Core) -> Option<OD> {
    if str == "" {
        return Some(OD::EMPTY);
    } else if str.contains("(") {
        // 如果有括号，则一定是内存型，取该地址的上的值返回过去
        // 内存型一共有9种，但是大体上还是 a(reg1,reg2,scal)这种类型
        return Some(OD::M_REG(parse_mm_ist(str, core)));
    } else if str.starts_with("$") {
        // 以$开头，则一定是立即数型，直接将数值放进去
        return Some(OD::IMM(hex_str2u(str)));
    } else if str.starts_with("%") {
        // 以%开头，则是寄存器型，要取寄存器的值
        let od = Some(OD::REG64(core.get_reg_value(str).unwrap(), str.to_string()));
        return od;
    } else {
        //是最后一种类型 立即数值型，取地址的值
        return Some(OD::M_IMM(hex_str2u(str)));
    }
}

// 将指令写入硬盘
fn write_inst(insts: Vec<&str>, pa: u64) {
    for num in 0..insts.len() {
        write_inst_dram(va2pa(pa + (0xc0 * num) as u64).unwrap(), insts[num]);
    }
}

// 更新pc
fn update_pc(core: &mut Core) {
    unsafe {
        core.rip.rip = core.rip.rip + 0xc0;
    }
}

// mov 指令 src dst,od有很多种格式，如果是立即数形式则直接把数字交给dst就好，但是如何判断是哪个还是很有问题的
fn mov_handler(src: OD, dst: OD, core: &mut Core) {
    let src_value = match src {
        OD::EMPTY => {
            panic!("bad parm in mov handler")
        }
        OD::IMM(value) => value,
        // OD::REG(addr) => read64bits_dram(va2pa(addr).unwrap()).unwrap(),
        OD::REG64(value, _) => value,
        OD::M_IMM(addr) => read64bits_dram(va2pa(addr).unwrap()).unwrap(),
        OD::M_REG(addr) => read64bits_dram(va2pa(addr).unwrap()).unwrap(),
    };
    println!("{:x}", src_value);
    match dst {
        OD::EMPTY => {
            panic!("bad dst type in mov inst")
        }
        OD::IMM(_) => {
            panic!("bad dst type in mov inst")
        }
        // OD::REG(addr) => write64bits_dram(va2pa(addr).unwrap(), src_value),
        OD::REG64(value, string) => {
            println!("{}", &string.as_str()[1..]);
            core.update_reg(&string.as_str()[1..], src_value)
        }
        OD::M_IMM(addr) => write64bits_dram(va2pa(addr).unwrap(), src_value),
        OD::M_REG(addr) => write64bits_dram(va2pa(addr).unwrap(), src_value),
    };
    update_pc(core);
}

// push 指令 mov %rxx (%rsp)，然后rsp-8
fn push_handler(src: OD, core: &mut Core) {
    let src_value = match src {
        OD::M_REG(addr) => read64bits_dram(va2pa(addr).unwrap()).unwrap(),
        OD::REG64(value, _) => value,
        _ => {
            panic!("bad src in push handler")
        }
    };
    unsafe {
        core.rsp.rsp -= 8;
        write64bits_dram(va2pa(core.rsp.rsp).unwrap(), src_value);
    }
    update_pc(core);
}

// 与push 相反 ，mov %(rsp) %rbp rsp += 8
fn pop_handler(src: OD, core: &mut Core) {
    unsafe {
        let src_value = read64bits_dram(va2pa(core.rsp.rsp).unwrap());
        match src {
            OD::REG64(value, string) => core.update_reg(string.as_str(), value),
            _ => panic!("bad pop inst"),
        }
        core.rsp.rsp += 8;
    }
    update_pc(core)
}

// add
fn add_handler(src: OD, dst: OD, core: &mut Core) {
    let src_value = match src {
        OD::EMPTY => panic!("bad inst src in add"),
        OD::IMM(_) => 0,
        OD::REG64(value, _) => value,
        OD::M_IMM(addr) => read64bits_dram(va2pa(addr).unwrap()).unwrap(),
        OD::M_REG(addr) => read64bits_dram(va2pa(addr).unwrap()).unwrap(),
    };
}

// call 其实就是将结果给rip
fn call_handler(src: OD, core: &mut Core) {
    let src_value = match src {
        OD::EMPTY => panic!("bad src in call inst"),
        OD::IMM(value) => value,
        OD::REG64(value, _) => value,
        OD::M_IMM(addr) => read64bits_dram(va2pa(addr).unwrap()).unwrap(),
        OD::M_REG(addr) => read64bits_dram(va2pa(addr).unwrap()).unwrap(),
    };
    core.update_reg("rip", src_value);
    // 同时栈+8
    unsafe {
        core.rsp.rsp -= 8;
    }
}

// 执行指令
fn oper_inst(inst: Inst, core: &mut Core) {
    match &inst.inst_type {
        INST_TYPE::MOV => {
            mov_handler(inst.src, inst.dst, core);
        }
        INST_TYPE::PUSH => {
            push_handler(inst.src, core);
        }
        INST_TYPE::POP => {
            pop_handler(inst.src, core);
        }
        INST_TYPE::LEAVE => todo!(),
        INST_TYPE::CALL => call_handler(inst.src, core),
        INST_TYPE::RET => todo!(),
        INST_TYPE::ADD => todo!(),
        INST_TYPE::SUB => todo!(),
        INST_TYPE::CMP => todo!(),
        INST_TYPE::JNE => todo!(),
        INST_TYPE::JMP => todo!(),
        default => {}
    }
}

fn callq_handler(core: &mut Core) {}

fn str_to_inst(str: &str, core: &mut Core) -> Inst {
    println!("str to inst : {}", &str);
    let z: Vec<&str> = str.split(&[' ', ','][..]).collect();
    let z: Vec<&str> = z.into_iter().filter(|&s| s != "").collect();
    let mut oper_str = "";
    let mut src_str = "";
    let mut dst_str = "";
    match z.len() {
        1 => {
            oper_str = z[0];
        }
        2 => {
            oper_str = z[0];
            src_str = z[1];
        }
        3 => {
            oper_str = z[0];
            src_str = z[1];
            dst_str = z[2];
        }
        _ => {
            panic!("error in parse full inst str")
        }
    }
    let inst = Inst {
        inst_type: parse_inst_type(oper_str).unwrap(),
        src: parse_od_type(src_str, &core).unwrap(),
        dst: parse_od_type(dst_str, &core).unwrap(),
    };
    return inst;
}

#[cfg(test)]
mod tests {
    use super::{parse_inst_type, parse_od_type, Core, Inst};
    use crate::hardware::ccpu::isa::*;
    use std::i64;

    #[test]
    fn test() {
        let f = RAX_REG {
            rax: 0x1234abcdff11ff11,
        };
        unsafe {
            let e = f.eax;
        }
    }

    #[test]
    fn test_parse_mm_num() {
        let mut core = Core::new();
        core.rax.eax = 0x100;
        core.rcx.ecx = 0x1;
        core.rdx.edx = 0x3;
        let str = "9(%eax,%edx)";
        // println!("{:x}",parse_mm_ist(str, &core));
        assert_eq!(0x100, parse_mm_ist("(%eax)", &core));
        assert_eq!(0x104, parse_mm_ist("4(%eax)", &core));
        assert_eq!(0x10c, parse_mm_ist(" 9( %eax , %edx)", &core));
        assert_eq!(0x108, parse_mm_ist("260(%ecx,%edx)", &core));
        assert_eq!(0xfc + 4, parse_mm_ist("0xfc(,%ecx,4)", &core));
        assert_eq!(0x10c, parse_mm_ist("(%eax,%edx,4)", &core));
    }

    #[test]
    fn test_reg() {
        let u1 = RAX_REG {
            rax: 0x12345678ffffabcd,
        };
        unsafe {
            // 应为十六进制有16个，所以需要2的4次方个0，1来表示，所以是4bit一个字符
            assert_eq!(u1.rax, 0x12345678ffffabcd);
            assert_eq!(u1.eax, 0xffffabcd);
            assert_eq!(u1.ax, 0xabcd);
            println!("{:x}", u1.inner.ah);
            println!("{:x}", u1.inner.al);
            assert_eq!(u1.inner.ah, 0xab);
            assert_eq!(u1.inner.al, 0xcd);
        };
    }

    #[test]
    fn test_inst_type_parse() {
        let i1: Vec<&str> = "mov call jne add".split(" ").collect();
        assert_eq!(INST_TYPE::MOV, parse_inst_type(i1[0]).unwrap());
        assert_eq!(INST_TYPE::CALL, parse_inst_type(i1[1]).unwrap());
        assert_eq!(INST_TYPE::JNE, parse_inst_type(i1[2]).unwrap());
        assert_eq!(INST_TYPE::ADD, parse_inst_type(i1[3]).unwrap());
    }

    #[test]
    fn test_mov() {
        // 立即数$，寄存器 %reg，内存 % 三种取,其中，立即数取得是数本身，不需要去地址取
        // 想要的效果就是一条指令，根据空格分成三份，分别进行解析，解析出mov,src地址，dst地址
        let insts_vec = vec![
            "push   %rbp",             // 0
            "mov    %rsp,%rbp",        // 1
            "mov    %rdi,-0x18(%rbp)", // 2
            "mov    %rsi,-0x20(%rbp)", // 3
            "mov    -0x18(%rbp),%rdx", // 4
            "mov    -0x20(%rbp),%rax", // 5
            "add    %rdx,%rax",        // 6
            "mov    %rax,-0x8(%rbp)",  // 7
            "mov    -0x8(%rbp),%rax",  // 8
            "pop    %rbp",             // 9
            "retq",                    // 10
            "mov    %rdx,%rsi",        // 11
            "mov    %rax,%rdi",        // 12
            "callq  $0x00400000",       // 13
            "mov    %rax,-0x8(%rbp)",  // 14
        ];
        // println!("{:?},{:?},{:?}",oper_str,src_str,dst_str);
        write_inst(insts_vec, 0x5574d795f020);
        let mut core = Core::new();
        core.rax.rax = 0x12340000;
        core.rbx.rbx = 0x0;
        core.rcx.rcx = 0x8000660;
        core.rdx.rdx = 0xabcd;
        core.rsi.rsi = 0x7ffffffee2f8;
        core.rdi.rdi = 0x1;
        core.rbp.rbp = 0x7ffffffee210;
        core.rsp.rsp = 0x7ffffffee1f0;
        core.rip.rip = 0x5574d795f860;
        write64bits_dram(va2pa(0x00007ffffffee210).unwrap(), 0x0000000008000660); // rbp
        write64bits_dram(va2pa(0x00007ffffffee200).unwrap(), 0xabcd);
        write64bits_dram(va2pa(0x00007ffffffee1f8).unwrap(), 0x12340000);
        write64bits_dram(va2pa(0x00007ffffffee1f0).unwrap(), 0x8000660);

        // ************************从这开始 准备好了数据*************
        // 取指 译码 执行
        let mut pa_addr = 0;
        unsafe {
            pa_addr = va2pa(core.rip.rip).unwrap();
            // println!("{}",pa_addr);
        }
        let inst = str_to_inst(read_inst_dram(pa_addr).unwrap().as_str(), &mut core);

        println!("{:?},{:?},{:?}", inst.inst_type, inst.src, inst.dst);
        // 现在是拿到了解析好的指令，开始执行
        oper_inst(inst, &mut core);
        // // mov执行结束后对比两者寄存器的变化
        unsafe {
            assert_eq!(0x12340000, core.rax.rax);
            assert_eq!(0x0, core.rbx.rbx);
            assert_eq!(0x8000660, core.rcx.rcx);
            assert_eq!(0xabcd, core.rdx.rdx);
            assert_eq!(0xabcd, core.rsi.rsi);
            assert_eq!(0x1, core.rdi.rdi);
            assert_eq!(0x7ffffffee210, core.rbp.rbp);
            assert_eq!(0x7ffffffee1f0, core.rsp.rsp);
            assert_eq!(0x5574d795f920, core.rip.rip);
        }
    }

    #[test]
    fn test_call() {
        let insts_vec = vec![
            "push   %rbp",             // 0  5574d795f020
            "mov    %rsp,%rbp",        // 1  5574d795f0e0
            "mov    %rdi,-0x18(%rbp)", // 2  5574d795f1a0
            "mov    %rsi,-0x20(%rbp)", // 3  5574D795F260
            "mov    -0x18(%rbp),%rdx", // 4  5574D795F320
            "mov    -0x20(%rbp),%rax", // 5  5574d795f3e0
            "add    %rdx,%rax",        // 6  5574D795F4A0
            "mov    %rax,-0x8(%rbp)",  // 7  5574D795F560
            "mov    -0x8(%rbp),%rax",  // 8  5574D795F620
            "pop    %rbp",             // 9  5574D795F6E0
            "retq",                    // 10 5574D795F7A0
            "mov    %rdx,%rsi",        // 11 5574d795f860  <= rip
            "mov    %rax,%rdi",        // 12 5574d795f920
            "callq  $0x5574d795f020",   // 13 5574d795f9e0
            "mov    %rax,-0x8(%rbp)",  // 14 5574d795faa0
        ];
        // println!("{:?},{:?},{:?}",oper_str,src_str,dst_str);
        write_inst(insts_vec, 0x5574d795f020);
        let mut core = Core::new();
        core.rax.rax = 0x12340000;
        core.rbx.rbx = 0x0;
        core.rcx.rcx = 0x8000660;
        core.rdx.rdx = 0xabcd;
        core.rsi.rsi = 0xabcd;
        core.rdi.rdi = 0x12340000;
        core.rbp.rbp = 0x7ffffffee210;
        core.rsp.rsp = 0x7ffffffee1f0;
        core.rip.rip = 0x5574d795f9e0;
        write64bits_dram(va2pa(0x00007ffffffee210).unwrap(), 0x0000000008000660); // rbp
        write64bits_dram(va2pa(0x00007ffffffee200).unwrap(), 0xabcd);
        write64bits_dram(va2pa(0x00007ffffffee1f8).unwrap(), 0x12340000);
        write64bits_dram(va2pa(0x00007ffffffee1f0).unwrap(), 0x8000660);

        // ************************从这开始 准备好了数据*************

        // 取指 译码 执行
        let mut pa_addr = 0;
        unsafe {
            pa_addr = va2pa(core.rip.rip).unwrap();
            println!("{}", pa_addr);
        }
        let inst = str_to_inst(read_inst_dram(pa_addr).unwrap().as_str(), &mut core);

        println!("{:?},{:?},{:?}", inst.inst_type, inst.src, inst.dst);
        // 现在是拿到了解析好的指令，开始执行
        oper_inst(inst, &mut core);

        unsafe {
            assert_eq!(0x12340000, core.rax.rax);
            assert_eq!(0x0, core.rbx.rbx);
            assert_eq!(0x8000660, core.rcx.rcx);
            assert_eq!(0xabcd, core.rdx.rdx);
            assert_eq!(0xabcd, core.rsi.rsi);
            assert_eq!(0x12340000, core.rdi.rdi);
            assert_eq!(0x7ffffffee210, core.rbp.rbp);
            assert_eq!(0x7ffffffee1e8, core.rsp.rsp);
            assert_eq!(0x5574d795f020, core.rip.rip);
        }
    }

    #[test]
    fn test_inst_cycle() {
        // 现在是拿到了解析好的指令，开始执行
    }
}
