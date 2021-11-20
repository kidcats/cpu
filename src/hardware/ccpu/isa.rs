#![allow(unused)]

use super::mmu::va2pa;
use crate::hardware::ccpu::core::Core;
use crate::hardware::memory::dram::*;
use core::fmt;
use std::{collections::btree_set::Union, default, fmt::Result, mem::size_of, rc::Rc, string};

#[derive(Clone)]
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

impl fmt::Debug for OD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result {
        match self {
            Self::EMPTY => write!(f, "EMPTY"),
            Self::IMM(arg0) => write!(f, "IMM {:x}", arg0),
            Self::REG64(arg0, arg1) => write!(f, "REG{} ({:x})", arg1, arg0),
            Self::M_IMM(arg0) => write!(f, "M_IMM {:x}", arg0),
            Self::M_REG(arg0) => write!(f, "M_REG {:x}", arg0),
        }
    }
}

// 需要的指令类型
#[allow(non_camel_case_types)]
#[derive(PartialEq)]
enum INST_TYPE {
    MOV,
    PUSH,
    POP,
    LEAVEQ,
    CALL,
    RET,
    ADD,
    SUB,
    CMPQ,
    JNE,
    JMP,
}

impl fmt::Debug for INST_TYPE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result {
        match self {
            Self::MOV => write!(f, "MOV"),
            Self::PUSH => write!(f, "PUSH"),
            Self::POP => write!(f, "POP"),
            Self::LEAVEQ => write!(f, "LEAVEQ"),
            Self::CALL => write!(f, "CALL"),
            Self::RET => write!(f, "RET"),
            Self::ADD => write!(f, "ADD"),
            Self::SUB => write!(f, "SUB"),
            Self::CMPQ => write!(f, "CMPQ"),
            Self::JNE => write!(f, "JNE"),
            Self::JMP => write!(f, "JMP"),
        }
    }
}

#[derive(Debug)]
struct Inst {
    inst_type: INST_TYPE,
    src: OD, // mov %reg %(0x188773)
    dst: OD,
}

// 将输入的字符串解析成对应的inst结构
fn parse_inst_type(str: &str) -> Option<INST_TYPE> {
    match str.as_ref() {
        "mov" => Some(INST_TYPE::MOV),
        "push" => Some(INST_TYPE::PUSH),
        "pop" => Some(INST_TYPE::POP),
        "leaveq" => Some(INST_TYPE::LEAVEQ),
        "callq" => Some(INST_TYPE::CALL),
        "retq" => Some(INST_TYPE::RET),
        "add" => Some(INST_TYPE::ADD),
        "sub" => Some(INST_TYPE::SUB),
        "cmpq" => Some(INST_TYPE::CMPQ),
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
pub fn hex_str2u(str: &str) -> u64 {
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
fn icalu(n1: i64, n2: u64, is_neg: bool) -> u64 {
    let n_temp = n1.abs() as u64;
    let result = if !is_neg { n2 + n_temp } else { n2 - n_temp };
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
    // println!("temp : {}", &temp);
    let temp2 = hex_str2i(imm.as_str());
    icalu(temp2, temp, imm_is_neg)
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

// 更新pc
fn update_pc(core: &mut Core) {
    unsafe {
        core.rip.rip = core.rip.rip + 0x40;
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
    // println!("{:x}", src_value);
    match dst {
        OD::EMPTY => {
            panic!("bad dst type in mov inst")
        }
        OD::IMM(_) => {
            panic!("bad dst type in mov inst")
        }
        // OD::REG(addr) => write64bits_dram(va2pa(addr).unwrap(), src_value),
        OD::REG64(value, string) => {
            println!("1c8  {:x}", src_value);
            core.update_reg(&string.as_str()[1..], src_value)
        }
        OD::M_IMM(addr) => write64bits_dram(va2pa(addr).unwrap(), src_value),
        OD::M_REG(addr) => write64bits_dram(va2pa(addr).unwrap(), src_value),
    };
    update_pc(core);
    core.flags_reset();
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
    core.flags_reset();
}

// 与push 相反 ，mov %(rsp) %rbp rsp += 8
fn pop_handler(src: OD, core: &mut Core) {
    unsafe {
        let src_value = read64bits_dram(va2pa(core.rsp.rsp).unwrap());
        println!("pop src value {:x}", src_value.unwrap());
        match src {
            OD::REG64(value, string) => core.update_reg(&string.as_str()[1..], src_value.unwrap()),
            _ => panic!("bad pop inst"),
        }
        core.rsp.rsp += 8;
    }
    core.flags_reset();
    update_pc(core)
}

// add
fn add_handler(src: OD, dst: OD, core: &mut Core) {
    let src_value = match src {
        OD::EMPTY => panic!("bad inst src in add"),
        OD::IMM(value) => value,
        OD::REG64(value, _) => value,
        OD::M_IMM(addr) => read64bits_dram(va2pa(addr).unwrap()).unwrap(),
        OD::M_REG(addr) => read64bits_dram(va2pa(addr).unwrap()).unwrap(),
    };
    let src_sign = (src_value >> 63) & 0x1;
    let mut dst_sign = 0;
    let mut val_sign = 0;
    let mut val = 0;
    match dst {
        OD::EMPTY => panic!("bad inst dst in add"),
        OD::IMM(_) => todo!(),
        OD::REG64(value, string) => {
            let dst_value = core.get_reg_value(string.as_str()).unwrap();
            val = dst_value + src_value;
            core.update_reg(&string.as_str()[1..], dst_value + src_value);
            dst_sign = (dst_value >> 63) & 0x1;
            val_sign = (val >> 63) & 0x1;
        }
        OD::M_IMM(_) => todo!(),
        OD::M_REG(_) => todo!(),
    }
    update_pc(core);
    core.flags.cf = val < src_value;
    core.flags.zf = val == 0;
    core.flags.sf = val_sign == 1;
    core.flags.of = ((src_sign == 1 && dst_sign == 1) && val_sign == 0)
        || ((src_sign == 0 && dst_sign == 0) && val_sign == 1);
}

// call 其实就是将结果给rip,同时把当前rip的值写入
fn call_handler(src: OD, core: &mut Core) {
    let src_value = match src {
        OD::EMPTY => panic!("bad src in call inst"),
        OD::IMM(value) => value,
        OD::REG64(value, _) => value,
        OD::M_IMM(addr) => read64bits_dram(va2pa(addr).unwrap()).unwrap(),
        OD::M_REG(addr) => read64bits_dram(va2pa(addr).unwrap()).unwrap(),
    };
    // 同时栈+8
    unsafe {
        core.rsp.rsp -= 8;
        println!("in cal {:x}", core.rsp.rsp);
        write64bits_dram(
            va2pa(core.rsp.rsp).unwrap(),
            core.get_reg_value("%rip").unwrap() + 0x40,
        );
    }
    core.update_reg("rip", src_value);
    // 将下一条指令写入的内容写入
    core.flags_reset();
}

// 本质上就是取出rsp地址上的值给rip，然后rsp+8
fn retq_handler(core: &mut Core) {
    unsafe {
        let va_addr = core.rsp.rsp;
        println!("retq va_addr value {:x}", va_addr);
        println!(
            "retq  value {:x}",
            read64bits_dram(va2pa(va_addr).unwrap()).unwrap()
        );
        core.update_reg("rip", read64bits_dram(va2pa(va_addr).unwrap()).unwrap());
        core.update_reg("rsp", va_addr + 8);
    }
    core.flags_reset();
}
// 与add 相反
fn sub_handler(src: OD, dst: OD, core: &mut Core) {
    let src_value = match src {
        OD::EMPTY => panic!("bad inst src in add"),
        OD::IMM(value) => value,
        OD::REG64(value, _) => value,
        OD::M_IMM(addr) => read64bits_dram(va2pa(addr).unwrap()).unwrap(),
        OD::M_REG(addr) => read64bits_dram(va2pa(addr).unwrap()).unwrap(),
    };
    let mut dst_sign = 0;
    let mut dst_value = 0;
    let src_sign = src_value >> 63 & 0x1;
    let mut val_sign = 0;
    let mut val = 0;
    match dst {
        OD::EMPTY => panic!("bad inst dst in add"),
        OD::IMM(_) => todo!(),
        OD::REG64(value, string) => {
            dst_value = core.get_reg_value(string.as_str()).unwrap();
            val = dst_value - src_value;
            dst_sign = (dst_value >> 63) & 0x1;
            val_sign = (val >> 63) & 0x1;
            core.update_reg(&string.as_str()[1..], dst_value - src_value);
        }
        OD::M_IMM(_) => todo!(),
        OD::M_REG(_) => todo!(),
    }
    update_pc(core);
    core.flags.cf = val > dst_value;
    core.flags.zf = val == 0;
    core.flags.sf = val_sign == 1;
    core.flags.of = ((src_sign == 1 && dst_sign == 0) && val_sign == 1)
        || ((src_sign == 0 && dst_sign == 1) && val_sign == 0);
}

fn cmpq_handler(src: OD, dst: OD, core: &mut Core) {
    let src_value = match src {
        OD::EMPTY => todo!(),
        OD::IMM(value) => value,
        OD::REG64(value, _) => value,
        OD::M_IMM(_) => todo!(),
        OD::M_REG(_) => todo!(),
    };
    let mut dst_sign = 0;
    let mut dst_value = 0;
    let src_sign = src_value >> 63 & 0x1;
    let mut val_sign = 0;
    let mut val = 0;
    match dst {
        OD::EMPTY => todo!(),
        OD::IMM(_) => todo!(),
        OD::REG64(_, _) => todo!(),
        OD::M_IMM(va_addr) => {
            dst_value = read64bits_dram(va2pa(va_addr).unwrap()).unwrap();
            val = dst_value - src_value;
            dst_sign = (dst_value >> 63) & 0x1;
            val_sign = (val >> 63) & 0x1;
        }
        OD::M_REG(va_addr) => {
            dst_value = read64bits_dram(va2pa(va_addr).unwrap()).unwrap();
            val = dst_value - src_value;
            dst_sign = (dst_value >> 63) & 0x1;
            val_sign = (val >> 63) & 0x1;
        }
    }

    update_pc(core);
    core.flags.cf = val > dst_value;
    core.flags.zf = val == 0;
    core.flags.sf = val_sign == 1;
    core.flags.of = ((src_sign == 1 && dst_sign == 0) && val_sign == 1)
        || ((src_sign == 0 && dst_sign == 1) && val_sign == 0);
}

fn jne_handler(src: OD, dst: OD, core: &mut Core) {
    let src_value = match src {
        OD::EMPTY => todo!(),
        OD::IMM(value) => value,
        OD::REG64(_, _) => todo!(),
        OD::M_IMM(_) => todo!(),
        OD::M_REG(_) => todo!(),
    };
    if !core.flags.zf {
        core.update_reg("rip", src_value);
    } else {
        update_pc(core);
    }
}

fn jmp_handler(src: OD, dst: OD, core: &mut Core) {
    let src_value = match src {
        OD::EMPTY => todo!(),
        OD::IMM(value) => value,
        OD::REG64(_, _) => todo!(),
        OD::M_IMM(_) => todo!(),
        OD::M_REG(_) => todo!(),
    };
    core.update_reg("rip", src_value);
    core.flags_reset();
}

// mov rbp,rsp
// pop rbp
fn leaveq_handler(src: OD, dst: OD, core: &mut Core) {
    core.update_reg("rsp", core.get_reg_value("%rbp").unwrap());
    // pop %rbp
    let old_value = read64bits_dram(va2pa(core.get_reg_value("%rsp").unwrap()).unwrap()).unwrap();
    core.update_reg("rbp", old_value);
    core.update_reg("rsp", core.get_reg_value("%rsp").unwrap() + 8);
    update_pc(core);
    core.flags_reset();
}

// 执行指令
fn oper_inst(inst: Inst, core: &mut Core) {
    match &inst.inst_type {
        INST_TYPE::MOV => mov_handler(inst.src, inst.dst, core),
        INST_TYPE::PUSH => push_handler(inst.src, core),
        INST_TYPE::POP => pop_handler(inst.src, core),
        INST_TYPE::LEAVEQ => leaveq_handler(inst.src, inst.dst, core),
        INST_TYPE::CALL => call_handler(inst.src, core),
        INST_TYPE::RET => retq_handler(core),
        INST_TYPE::ADD => add_handler(inst.src, inst.dst, core),
        INST_TYPE::SUB => sub_handler(inst.src, inst.dst, core),
        INST_TYPE::CMPQ => cmpq_handler(inst.src, inst.dst, core),
        INST_TYPE::JNE => jne_handler(inst.src, inst.dst, core),
        INST_TYPE::JMP => jmp_handler(inst.src, inst.dst, core),
        _default => {}
    }
}

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
    use super::{parse_inst_type, parse_od_type, Inst};
    use crate::hardware::ccpu::core::*;
    use crate::hardware::ccpu::isa::*;
    use crate::hardware::memory::dram::write_inst;
    use std::i64;

    #[test]
    fn test() {
        let src_value: i64 = -100;
        let dst_value = 101;
        let val = src_value + dst_value;
        let src_sign = (src_value >> 63) & 0x1;
        let dst_sign = (dst_value >> 63) & 0x1;
        let val_sign = (val >> 63) & 0x1;
        assert_eq!(src_sign, 1);
        assert_eq!(dst_sign, 0);
        assert_eq!(val_sign, 0);
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
        assert_eq!(0x2, parse_mm_ist("-0x2(,%ecx,4)", &core));
        assert_eq!(0x10c, parse_mm_ist("(%eax,%edx,4)", &core));
    }

    #[test]
    fn test_inst_type_parse() {
        let i1: Vec<&str> = "mov callq jne add".split(" ").collect();
        assert_eq!(INST_TYPE::MOV, parse_inst_type(i1[0]).unwrap());
        assert_eq!(INST_TYPE::CALL, parse_inst_type(i1[1]).unwrap());
        assert_eq!(INST_TYPE::JNE, parse_inst_type(i1[2]).unwrap());
        assert_eq!(INST_TYPE::ADD, parse_inst_type(i1[3]).unwrap());
    }

    #[test]
    fn test_retq() {
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
        let mut core = Core::new();
        core.rax.rax = 0x12340000;
        core.rbx.rbx = 0x0;
        core.rcx.rcx = 0x8000660;
        core.rdx.rdx = 0xabcd;
        core.rsi.rsi = 0x7ffffffee2f8;
        core.rdi.rdi = 0x1;
        core.rbp.rbp = 0x7ffffffee210;
        core.rsp.rsp = 0x7ffffffee1f0;
        core.rip.rip = 0x5574d795f2e0;
        write64bits_dram(va2pa(0x00007ffffffee210).unwrap(), 0x0000000008000660); // rbp
        write64bits_dram(va2pa(0x00007ffffffee200).unwrap(), 0xabcd); // rbp
        write64bits_dram(va2pa(0x00007ffffffee1f8).unwrap(), 0x12340000);
        write64bits_dram(va2pa(0x00007ffffffee1f0).unwrap(), 0x8000660);

        // ************************从这开始 准备好了数据*************
        for i in 0..15 {
            // 循环从rip地址中取数据
            // 取指 译码 执行
            println!("*************************");
            let mut pa_addr = 0;
            unsafe {
                pa_addr = va2pa(core.rip.rip).unwrap();
                println!("{}", pa_addr);
            }
            let inst = str_to_inst(read_inst_dram(pa_addr).unwrap().as_str(), &mut core);

            println!("{:?},{:?},{:?}", inst.inst_type, inst.src, inst.dst);
            // 现在是拿到了解析好的指令，开始执行
            oper_inst(inst, &mut core);
            core.get_all_reg_value();
            println!(
                "ox1e8 value {:x}",
                read64bits_dram(va2pa(0x7ffffffee1c8).unwrap()).unwrap()
            );
        }

        unsafe {
            assert_eq!(0x1234abcd, core.rax.rax);
            assert_eq!(0x0, core.rbx.rbx);
            assert_eq!(0x8000660, core.rcx.rcx);
            assert_eq!(0x12340000, core.rdx.rdx);
            assert_eq!(0xabcd, core.rsi.rsi);
            assert_eq!(0x12340000, core.rdi.rdi);
            assert_eq!(0x7ffffffee210, core.rbp.rbp);
            assert_eq!(0x7ffffffee1f0, core.rsp.rsp);
            assert_eq!(0x5574d795f3e0, core.rip.rip);
        }
    }

    #[test]
    fn test_inst_cycle() {
        // 现在是拿到了解析好的指令，开始执行
        let insts_vec = vec![
            "push   %rbp                             ", // 0  0x400000
            "mov    %rsp,%rbp                        ", // 1  0x400040
            "sub    $0x10,%rsp                       ", // 2  0x400080
            "mov    %rdi,-0x8(%rbp)                  ", // 3  0x4000c0
            "cmpq   $0x0,-0x8(%rbp)                  ", // 4  0x400100
            "jne    $0x400200                        ", // 5: 0x400140 jump to 8
            "mov    $0x0,%eax                        ", // 6  0x400180
            "jmp    $0x400380                        ", // 7: 0x4001c0 jump to 14
            "mov    -0x8(%rbp),%rax                  ", // 8  0x400200
            "sub    $0x1,%rax                        ", // 9  0x400240
            "mov    %rax,%rdi                        ", // 10 0x400280
            "callq  $0x00400000                      ", // 11 0x4002c0
            "mov    -0x8(%rbp),%rdx                  ", // 12 0x400300
            "add    %rdx,%rax                        ", // 13 0x400340
            "leaveq                                  ", // 14 0x400380
            "retq                                    ", // 15 0x4003c0
            "mov    $0x3,%edi                        ", // 16 0x400400  rip
            "callq  $0x00400000                      ", // 17 0x400440
            "mov    %rax,-0x8(%rbp)                  ", // 18 0x400480
        ];
        write_inst(&insts_vec, 0x400000);
        let mut core = Core::new();
        core.rax.rax = 0x8000630;
        core.rbx.rbx = 0x0;
        core.rcx.rcx = 0x8000650;
        core.rdx.rdx = 0x7ffffffee328;
        core.rsi.rsi = 0x7ffffffee318;
        core.rdi.rdi = 0x1;
        core.rbp.rbp = 0x7ffffffee230;
        core.rsp.rsp = 0x7ffffffee220;
        core.rip.rip = 0x400400;
        write64bits_dram(va2pa(0x7ffffffee230).unwrap(), 0x8000650); // rbp
        write64bits_dram(va2pa(0x7ffffffee220).unwrap(), 0x7ffffffee310);

        // ************************从这开始 准备好了数据*************
        for i in 0..19 {
            // 循环从rip地址中取数据
            // 取指 译码 执行
            println!("*************************");
            let mut pa_addr = 0;
            unsafe {
                pa_addr = va2pa(core.rip.rip).unwrap();
                println!("{}", pa_addr);
            }
            let inst = str_to_inst(read_inst_dram(pa_addr).unwrap().as_str(), &mut core);

            println!("{:?},{:?},{:?}", inst.inst_type, inst.src, inst.dst);
            // 现在是拿到了解析好的指令，开始执行
            oper_inst(inst, &mut core);
            core.get_all_reg_value();
            core.get_all_flags();
        }
    }
}
