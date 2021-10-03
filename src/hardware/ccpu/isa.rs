#![allow(unused)]

use super::mmu::va2pa;
use crate::hardware::memory::dram::*;
use core::fmt;
use std::{collections::{btree_set::Union}, default, mem::size_of};

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

#[derive(Debug)]
enum OD {
    EMPTY,
    IMM(u64),
    REG(u64),
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
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    rbp: u64,
    rsp: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
    esi: u32,
    edi: u32,
    ebp: u32,
    esp: u32,
    ax: u16,
    bx: u16,
    cx: u16,
    dx: u16,
    si: u16,
    di: u16,
    bp: u16,
    sp: u16,
    al: u8,
    ah: u8,
    bl: u8,
    bh: u8,
    cl: u8,
    ch: u8,
    dl: u8,
    dh: u8,
    sil: u8,
    sih: u8,
    dil: u8,
    dih: u8,
    bpl: u8,
    bph: u8,
    spl: u8,
    sph: u8,
    r8d: u32,
    r9d: u32,
    r10d: u32,
    r11d: u32,
    r12d: u32,
    r13d: u32,
    r14d: u32,
    r15d: u32,
    r8w: u16,
    r9w: u16,
    r10w: u16,
    r11w: u16,
    r12w: u16,
    r13w: u16,
    r14w: u16,
    r15w: u16,
    r8b: u8,
    r9b: u8,
    r10b: u8,
    r11b: u8,
    r12b: u8,
    r13b: u8,
    r14b: u8,
    r15b: u8,
    rip: u64,
    eip: u32,
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
                rax: { regs.0.rax },
                rbx: { regs.1.rbx },
                rcx: { regs.2.rcx },
                rdx: { regs.3.rdx },
                rsi: { regs.4.rsi },
                rdi: { regs.5.rdi },
                rbp: { regs.6.rbp },
                rsp: { regs.7.rsp },
                r8: { regs.8.r8 },
                r9: { regs.9.r9 },
                r10: { regs.10.r10 },
                r11: { regs.11.r11 },
                r12: { regs.12.r12 },
                r13: { regs.13.r13 },
                r14: { regs.14.r14 },
                r15: { regs.15.r15 },
                eax: { regs.0.eax },
                ebx: { regs.1.ebx },
                ecx: { regs.2.ecx },
                edx: { regs.3.edx },
                esi: { regs.4.esi },
                edi: { regs.5.edi },
                ebp: { regs.6.ebp },
                esp: { regs.7.esp },
                ax: { regs.0.ax },
                bx: { regs.1.bx },
                cx: { regs.2.cx },
                dx: { regs.3.dx },
                si: { regs.4.si },
                di: { regs.5.di },
                bp: { regs.6.bp },
                sp: { regs.7.sp },
                al: { regs.0.inner.al },
                ah: { regs.0.inner.ah },
                bl: { regs.1.inner.bl },
                bh: { regs.1.inner.bh },
                cl: { regs.2.inner.cl },
                ch: { regs.2.inner.ch },
                dl: { regs.3.inner.dl },
                dh: { regs.3.inner.dh },
                sil: { regs.4.inner.sil },
                sih: { regs.4.inner.sih },
                dil: { regs.5.inner.dil },
                dih: { regs.5.inner.dih },
                bpl: { regs.6.inner.bpl },
                bph: { regs.6.inner.bph },
                spl: { regs.7.inner.spl },
                sph: { regs.7.inner.sph },
                r8d: { regs.8.r8d },
                r9d: { regs.9.r9d },
                r10d: { regs.10.r10d },
                r11d: { regs.11.r11d },
                r12d: { regs.12.r12d },
                r13d: { regs.13.r13d },
                r14d: { regs.14.r14d },
                r15d: { regs.15.r15d },
                r8w: { regs.8.r8w },
                r9w: { regs.9.r9w },
                r10w: { regs.10.r10w },
                r11w: { regs.11.r11w },
                r12w: { regs.12.r12w },
                r13w: { regs.13.r13w },
                r14w: { regs.14.r14w },
                r15w: { regs.15.r15w },
                r8b: { regs.8.r8b },
                r9b: { regs.9.r9b },
                r10b: { regs.10.r10b },
                r11b: { regs.11.r11b },
                r12b: { regs.12.r12b },
                r13b: { regs.13.r13b },
                r14b: { regs.14.r14b },
                r15b: { regs.15.r15b },
                rip : {regs.16.rip},
                eip : {regs.16.eip}
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
        "call" => Some(INST_TYPE::CALL),
        "ret" => Some(INST_TYPE::RET),
        "add" => Some(INST_TYPE::ADD),
        "sub" => Some(INST_TYPE::SUB),
        "cmp" => Some(INST_TYPE::CMP),
        "jne" => Some(INST_TYPE::JNE),
        "jmp" => Some(INST_TYPE::JMP),
        default => None,
    }
}

// 将寄存器的值解析出来 要带%
fn reg2value(str: &str, core: &Core) -> Option<u64> {
    let od = match str.as_ref() {
        "%rax" => Some(core.rax),
        "%rbx" => Some(core.rbx),
        "%rcx" => Some(core.rcx),
        "%rdx" => Some(core.rdx),
        "%rsi" => Some(core.rsi),
        "%rdi" => Some(core.rdi),
        "%rbp" => Some(core.rbp),
        "%rsp" => Some(core.rsp),
        "%eax" => Some(core.eax as u64),
        "%ebx" => Some(core.ebx as u64),
        "%ecx" => Some(core.ecx as u64),
        "%edx" => Some(core.edx as u64),
        "%esi" => Some(core.esi as u64),
        "%edi" => Some(core.edi as u64),
        "%ebp" => Some(core.ebp as u64),
        "%esp" => Some(core.esp as u64),
        "%ax" => Some(core.ax as u64),
        "%bx" => Some(core.bx as u64),
        "%cx" => Some(core.cx as u64),
        "%dx" => Some(core.dx as u64),
        "%si" => Some(core.si as u64),
        "%di" => Some(core.di as u64),
        "%bp" => Some(core.bp as u64),
        "%sp" => Some(core.sp as u64),
        "%al" => Some(core.al as u64),
        "%ah" => Some(core.ah as u64),
        "%bl" => Some(core.bl as u64),
        "%bh" => Some(core.bh as u64),
        "%cl" => Some(core.cl as u64),
        "%ch" => Some(core.ch as u64),
        "%dl" => Some(core.dl as u64),
        "%dh" => Some(core.dh as u64),
        "%sil" => Some(core.sil as u64),
        "%sih" => Some(core.sih as u64),
        "%dil" => Some(core.dil as u64),
        "%dih" => Some(core.dih as u64),
        "%bpl" => Some(core.bpl as u64),
        "%bph" => Some(core.bph as u64),
        "%spl" => Some(core.spl as u64),
        "%sph" => Some(core.sph as u64),
        "%r8" => Some(core.r8),
        "%r9" => Some(core.r9),
        "%r10" => Some(core.r10),
        "%r11" => Some(core.r11),
        "%r12" => Some(core.r12),
        "%r13" => Some(core.r13),
        "%r14" => Some(core.r14),
        "%r15" => Some(core.r15),
        "%r8d" => Some(core.r8d as u64),
        "%r9d" => Some(core.r9d as u64),
        "%r10d" => Some(core.r10d as u64),
        "%r11d" => Some(core.r11d as u64),
        "%r12d" => Some(core.r12d as u64),
        "%r13d" => Some(core.r13d as u64),
        "%r14d" => Some(core.r14d as u64),
        "%r15d" => Some(core.r15d as u64),
        "%r8w" => Some(core.r8w as u64),
        "%r9w" => Some(core.r9w as u64),
        "%r10w" => Some(core.r10w as u64),
        "%r11w" => Some(core.r11w as u64),
        "%r12w" => Some(core.r12w as u64),
        "%r13w" => Some(core.r13w as u64),
        "%r14w" => Some(core.r14w as u64),
        "%r15w" => Some(core.r15w as u64),
        "%r8b" => Some(core.r8b as u64),
        "%r9b" => Some(core.r9b as u64),
        "%r10b" => Some(core.r10b as u64),
        "%r11b" => Some(core.r11b as u64),
        "%r12b" => Some(core.r12b as u64),
        "%r13b" => Some(core.r13b as u64),
        "%r14b" => Some(core.r14b as u64),
        "%r15b" => Some(core.r15b as u64),
        "%rip" => Some(core.rip as u64),
        "%eip" => Some(core.eip as u64),
        default => Some(0 as u64),
    };
    return od;
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
    let temp =
        reg2value(r1.as_str(), core).unwrap() + reg2value(r2.as_str(), core).unwrap() * scal_temp;
    let temp2 = hex_str2i(imm.as_str());
    icalu(temp2, temp)
}

// 解析操作数的类型
fn parse_od_type(str: &str, core: &Core) -> Option<OD> {
    if str == "" {
        return Some(OD::EMPTY);
    }else if str.contains("(") {
        // 如果有括号，则一定是内存型，取地址的值
        // 内存型一共有9种，但是大体上还是 a(reg1,reg2,scal)这种类型
        return Some(OD::M_REG(parse_mm_ist(str, core)));
    } else if str.starts_with("$") {
        // 以$开头，则一定是立即数型，直接将数值放进去
        return Some(OD::IMM(hex_str2u(str)));
    } else if str.starts_with("%") {
        // 以%开头，则是寄存器型，要取地址的值
        let od = match str.as_ref() {
            "%rax" => Some(OD::REG(core.rax)),
            "%rbx" => Some(OD::REG(core.rbx)),
            "%rcx" => Some(OD::REG(core.rcx)),
            "%rdx" => Some(OD::REG(core.rdx)),
            "%rsi" => Some(OD::REG(core.rsi)),
            "%rdi" => Some(OD::REG(core.rdi)),
            "%rbp" => Some(OD::REG(core.rbp)),
            "%rsp" => Some(OD::REG(core.rsp)),
            "%eax" => Some(OD::REG(core.eax as u64)),
            "%ebx" => Some(OD::REG(core.ebx as u64)),
            "%ecx" => Some(OD::REG(core.ecx as u64)),
            "%edx" => Some(OD::REG(core.edx as u64)),
            "%esi" => Some(OD::REG(core.esi as u64)),
            "%edi" => Some(OD::REG(core.edi as u64)),
            "%ebp" => Some(OD::REG(core.ebp as u64)),
            "%esp" => Some(OD::REG(core.esp as u64)),
            "%ax" => Some(OD::REG(core.ax as u64)),
            "%bx" => Some(OD::REG(core.bx as u64)),
            "%cx" => Some(OD::REG(core.cx as u64)),
            "%dx" => Some(OD::REG(core.dx as u64)),
            "%si" => Some(OD::REG(core.si as u64)),
            "%di" => Some(OD::REG(core.di as u64)),
            "%bp" => Some(OD::REG(core.bp as u64)),
            "%sp" => Some(OD::REG(core.sp as u64)),
            "%al" => Some(OD::REG(core.al as u64)),
            "%ah" => Some(OD::REG(core.ah as u64)),
            "%bl" => Some(OD::REG(core.bl as u64)),
            "%bh" => Some(OD::REG(core.bh as u64)),
            "%cl" => Some(OD::REG(core.cl as u64)),
            "%ch" => Some(OD::REG(core.ch as u64)),
            "%dl" => Some(OD::REG(core.dl as u64)),
            "%dh" => Some(OD::REG(core.dh as u64)),
            "%sil" => Some(OD::REG(core.sil as u64)),
            "%sih" => Some(OD::REG(core.sih as u64)),
            "%dil" => Some(OD::REG(core.dil as u64)),
            "%dih" => Some(OD::REG(core.dih as u64)),
            "%bpl" => Some(OD::REG(core.bpl as u64)),
            "%bph" => Some(OD::REG(core.bph as u64)),
            "%spl" => Some(OD::REG(core.spl as u64)),
            "%sph" => Some(OD::REG(core.sph as u64)),
            "%r8" => Some(OD::REG(core.r8)),
            "%r9" => Some(OD::REG(core.r9)),
            "%r10" => Some(OD::REG(core.r10)),
            "%r11" => Some(OD::REG(core.r11)),
            "%r12" => Some(OD::REG(core.r12)),
            "%r13" => Some(OD::REG(core.r13)),
            "%r14" => Some(OD::REG(core.r14)),
            "%r15" => Some(OD::REG(core.r15)),
            "%r8d" => Some(OD::REG(core.r8d as u64)),
            "%r9d" => Some(OD::REG(core.r9d as u64)),
            "%r10d" => Some(OD::REG(core.r10d as u64)),
            "%r11d" => Some(OD::REG(core.r11d as u64)),
            "%r12d" => Some(OD::REG(core.r12d as u64)),
            "%r13d" => Some(OD::REG(core.r13d as u64)),
            "%r14d" => Some(OD::REG(core.r14d as u64)),
            "%r15d" => Some(OD::REG(core.r15d as u64)),
            "%r8w" => Some(OD::REG(core.r8w as u64)),
            "%r9w" => Some(OD::REG(core.r9w as u64)),
            "%r10w" => Some(OD::REG(core.r10w as u64)),
            "%r11w" => Some(OD::REG(core.r11w as u64)),
            "%r12w" => Some(OD::REG(core.r12w as u64)),
            "%r13w" => Some(OD::REG(core.r13w as u64)),
            "%r14w" => Some(OD::REG(core.r14w as u64)),
            "%r15w" => Some(OD::REG(core.r15w as u64)),
            "%r8b" => Some(OD::REG(core.r8b as u64)),
            "%r9b" => Some(OD::REG(core.r9b as u64)),
            "%r10b" => Some(OD::REG(core.r10b as u64)),
            "%r11b" => Some(OD::REG(core.r11b as u64)),
            "%r12b" => Some(OD::REG(core.r12b as u64)),
            "%r13b" => Some(OD::REG(core.r13b as u64)),
            "%r14b" => Some(OD::REG(core.r14b as u64)),
            "%r15b" => Some(OD::REG(core.r15b as u64)),
            default => None,
        };
        return od;
    } else {
        //是最后一种类型 立即数值型，取地址的值
        return Some(OD::M_IMM(hex_str2u(str)));
    }
}

// 更新pc
fn update_pc(core:&mut Core){
    core.rip = core.rip + 4 * 64;
}

// mov 指令 src dst
fn mov_handler(src: OD, dst: OD) {
    let src_value = match src {
        OD::EMPTY => 0,
        OD::IMM(value) => value,
        OD::REG(addr) => read64bits_dram(va2pa(addr).unwrap()).unwrap(),
        OD::M_IMM(addr) => read64bits_dram(va2pa(addr).unwrap()).unwrap(),
        OD::M_REG(addr) => read64bits_dram(va2pa(addr).unwrap()).unwrap(),
    };
    match dst {
        OD::EMPTY => {
            panic!("bad dst type in mov inst")
        }
        OD::IMM(_) => {
            panic!("bad dst type in mov inst")
        }
        OD::REG(addr) => write64bits_dram(va2pa(addr).unwrap(), src_value),
        OD::M_IMM(addr) => write64bits_dram(va2pa(addr).unwrap(), src_value),
        OD::M_REG(addr) => write64bits_dram(va2pa(addr).unwrap(), src_value),
    };
}

// push 指令
fn push_handler(){

}

// 执行指令
fn oper_inst(inst: &Inst) {
    match &inst.inst_type {
        INST_TYPE::MOV => {}
        INST_TYPE::PUSH => todo!(),
        INST_TYPE::POP => todo!(),
        INST_TYPE::LEAVE => todo!(),
        INST_TYPE::CALL => todo!(),
        INST_TYPE::RET => todo!(),
        INST_TYPE::ADD => todo!(),
        INST_TYPE::SUB => todo!(),
        INST_TYPE::CMP => todo!(),
        INST_TYPE::JNE => todo!(),
        INST_TYPE::JMP => todo!(),
        default => {}
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_inst_type, parse_od_type, Core, Inst};
    use crate::hardware::ccpu::isa::*;
    use std::i64;

    #[test]
    fn test() {
        let str = "260";
    }

    #[test]
    fn test_parse_mm_num() {
        let mut core = Core::new();
        core.eax = 0x100;
        core.ecx = 0x1;
        core.edx = 0x3;
        let str = "9(%eax,%edx)";
        // println!("{:x}",parse_mm_ist(str, &core));
        assert_eq!(0x100, parse_mm_ist("(%eax)", &core));
        assert_eq!(0x104, parse_mm_ist("4(%eax)", &core));
        assert_eq!(0x10c, parse_mm_ist(" 9( %eax , %edx)", &core));
        assert_eq!(0x108, parse_mm_ist("260(%ecx,%edx)", &core));
        assert_eq!(0x100, parse_mm_ist("0xfc(,%ecx,4)", &core));
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
    fn test_add() {
        // 立即数$，寄存器 %reg，内存 % 三种取,其中，立即数取得是数本身，不需要去地址取
        // 想要的效果就是一条指令，根据空格分成三份，分别进行解析，解析出mov,src地址，dst地址
        let inst1 = String::from("push   %rbp");
        let z: Vec<&str> = inst1.split(&[' ', ','][..]).collect().into_iter().filter();
        let mut oper_str = "";
        let mut src_str = "";
        let mut dst_str = "";
        println!("{:?}",z);
        match z.len(){
            1=>{
                oper_str = z[0];
            },
            2=>{
                oper_str = z[0];
                src_str = z[1];
            },
            3=>{
                oper_str = z[0];
                src_str = z[1];
                dst_str = z[2];
            },
            _=>{panic!("error in parse full inst str")}
        }
        let mut core = Core::new();
        core.rax = 0xabcd;
        core.rbx = 0x8000670;
        core.rcx = 0x8000670;
        core.rdx = 0x12340000;
        core.rsi = 0x7ffffffee208;
        core.rdi = 0x1;
        core.rbp = 0x7ffffffee110;
        core.rsp = 0x7ffffffee0f0;
        core.rip = 0x400300;
        write64bits_dram(va2pa(0x7ffffffee110).unwrap(), 0x0000000000000000); // rbp
        write64bits_dram(va2pa(0x7ffffffee108).unwrap(), 0x0000000000000000);
        write64bits_dram(va2pa(0x7ffffffee100).unwrap(), 0x0000000012340000);
        write64bits_dram(va2pa(0x7ffffffee0f8).unwrap(), 0x000000000000abcd);
        write64bits_dram(va2pa(0x7ffffffee0f0).unwrap(), 0x0000000000000000); // rsp
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
            "callq  0x00400000",       // 13
            "mov    %rax,-0x8(%rbp)",  // 14
        ];
        let inst = Inst {
            inst_type: parse_inst_type(oper_str).unwrap(),
            src: parse_od_type(src_str, &core).unwrap(),
            dst: parse_od_type(dst_str, &core).unwrap(),
        };
        // 现在是拿到了解析好的指令，开始执行
        oper_inst(&inst);
        // 执行结束后对比两者寄存器的变化
        // assert_eq!(0xabcd, core.rax);
        // assert_eq!(0x8000670, core.rbx);
        // assert_eq!(0x8000670, core.rcx);
        // assert_eq!(0x12340000, core.rdx);
        // assert_eq!(0x12340000, core.rsi);
        // assert_eq!(0xabcd, core.rdi);
        // assert_eq!(0x7ffffffee110, core.rbp);
        // assert_eq!(0x7ffffffee0e8, core.rsp);
        // assert_eq!(0x7ffffffee0e8, core.rip);
    }

    #[test]
    fn test_inst_cycle() {
        // 现在是拿到了解析好的指令，开始执行
    }
}
