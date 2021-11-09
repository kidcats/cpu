#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RAX_Inner {
    al: u8, // 要低地址在前，高地址在后
    ah: u8,
}
#[repr(C)]
pub union RAX_REG {
    pub rax: u64,
    pub eax: u32,
    ax: u16,
    inner: RAX_Inner,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RBX_Inner {
    bl: u8,
    bh: u8,
}
#[repr(C)]
pub union RBX_REG {
    inner: RBX_Inner,
    bx: u16,
    pub ebx: u32,
    pub rbx: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RCX_Inner {
    cl: u8,
    ch: u8,
}
#[repr(C)]
pub union RCX_REG {
    inner: RCX_Inner,
    cx: u16,
    pub ecx: u32,
    pub rcx: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RDX_Inner {
    dl: u8,
    dh: u8,
}

#[repr(C)]
pub union RDX_REG {
    inner: RDX_Inner,
    dx: u16,
    pub edx: u32,
    pub rdx: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RSI_Inner {
    sil: u8,
    sih: u8,
}
#[repr(C)]
pub union RSI_REG {
    inner: RSI_Inner,
    si: u16,
    esi: u32,
    pub rsi: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RDI_Inner {
    dil: u8,
    dih: u8,
}
#[repr(C)]
pub union RDI_REG {
    inner: RDI_Inner,
    di: u16,
    edi: u32,
    pub rdi: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RBP_Inner {
    bpl: u8,
    bph: u8,
}
#[repr(C)]
pub union RBP_REG {
    inner: RBP_Inner,
    bp: u16,
    ebp: u32,
    pub rbp: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RSP_Inner {
    spl: u8,
    sph: u8,
}
#[repr(C)]
pub union RSP_REG {
    inner: RSP_Inner,
    sp: u16,
    esp: u32,
    pub rsp: u64,
}

#[repr(C)]
pub union R8_REG {
    r8b: u8,
    r8w: u16,
    r8d: u32,
    r8: u64,
}

#[repr(C)]
pub union R9_REG {
    r9b: u8,
    r9w: u16,
    r9d: u32,
    r9: u64,
}
#[repr(C)]
pub union R10_REG {
    r10b: u8,
    r10w: u16,
    r10d: u32,
    r10: u64,
}
#[repr(C)]
pub union R11_REG {
    r11b: u8,
    r11w: u16,
    r11d: u32,
    r11: u64,
}
#[repr(C)]
pub union R12_REG {
    r12b: u8,
    r12w: u16,
    r12d: u32,
    r12: u64,
}
#[repr(C)]
pub union R13_REG {
    r13b: u8,
    r13w: u16,
    r13d: u32,
    r13: u64,
}
#[repr(C)]
pub union R14_REG {
    r14b: u8,
    r14w: u16,
    r14d: u32,
    r14: u64,
}
#[repr(C)]
pub union R15_REG {
    r15b: u8,
    r15w: u16,
    r15d: u32,
    r15: u64,
}

#[repr(C)]
pub union RIP_REG {
    pub rip: u64,
    eip: u32,
}

#[repr(C)]
pub struct CORE_FLAG {
    pub cf: bool,
    pub zf: bool,
    pub sf: bool,
    pub of: bool,
}

// core 里面保存和所有的寄存器，和符号信息,符号可以先不管
pub struct Core {
    pub rax: RAX_REG,
    pub rbx: RBX_REG,
    pub rcx: RCX_REG,
    pub rdx: RDX_REG,
    pub rsi: RSI_REG,
    pub rdi: RDI_REG,
    pub rbp: RBP_REG,
    pub rsp: RSP_REG,
    pub r8: R8_REG,
    pub r9: R9_REG,
    pub r10: R10_REG,
    pub r11: R11_REG,
    pub r12: R12_REG,
    pub r13: R13_REG,
    pub r14: R14_REG,
    pub r15: R15_REG,
    pub rip: RIP_REG,
    pub flags: CORE_FLAG,
}

impl Core {
    pub fn new() -> Core {
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
            flags : {
                CORE_FLAG { cf: false, zf: false, sf: false, of: false }
            },
        }
    }

    /**
     * update reg value
     */
    pub fn update_reg(&mut self, od: &str, value: u64) {
        println!("update_reg {}", od);
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
            "rdx" => self.rdx.rdx = value,
            "edx" => self.rdx.edx = value as u32,
            "dx" => self.rdx.dx = value as u16,
            "dh" => self.rdx.inner.dh = value as u8,
            "dl" => self.rdx.inner.dl = value as u8,
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
    
    pub fn get_reg_value(&self, name: &str) -> Option<u64> {
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
                _default => Some(0 as u64),
            }
        }
    }
    
    /**
     * printf all regs value for core
     */
    pub fn get_all_reg_value(&self) {
        unsafe {
            println!("rax : {:x}, rbx: {:x} rcx {:x} \n rdx {:x} rsi {:x} rdi {:x} \n rbp {:x} rsp {:x},rip {:x}",
            self.rax.rax,self.rbx.rbx,self.rcx.rcx,self.rdx.rdx,self.rsi.rsi,self.rdi.rdi,self.rbp.rbp,self.rsp.rsp,self.rip.rip)
        }
    }

    pub fn get_all_flags(&self){
        println!("{},{},{},{}",self.flags.zf,self.flags.of,self.flags.cf,self.flags.sf)
    }
    /** 
     * reset the core flags
    */
    pub fn flags_reset(&mut self){
        self.flags.cf = false;
        self.flags.zf = false;
        self.flags.of = false;
        self.flags.sf = false;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let f = RAX_REG {
            rax: 0x1234abcdff11ff11,
        };
        unsafe {
            let _e = f.eax;
        }
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
    fn test_flag(){
        let mut core = Core::new();
        core.flags.cf = true;
        core.flags.of = true;
        assert_eq!(true,core.flags.cf);
        assert_eq!(true,core.flags.of);
        assert_eq!(false,core.flags.sf);
        assert_eq!(false,core.flags.zf);
        core.flags_reset();
        assert_eq!(false,core.flags.cf);
        assert_eq!(false,core.flags.of);
        assert_eq!(false,core.flags.sf);
        assert_eq!(false,core.flags.zf);
    }
}
