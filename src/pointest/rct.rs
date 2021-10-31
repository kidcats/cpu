use std::{borrow::Borrow, cell::RefCell, rc::Rc};

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
struct Core {
    rax: RAX_REG,
}

impl Core {
    fn new() -> Self {
        let mut x = RAX_REG {
            rax: 0x0000_0000_0000_0000,
        };
        unsafe { Core { rax: x } }
    }

    fn update_reg(&mut self, od: &str, value: u64) {
        match od {
            "rax" => self.rax.rax = value,
            "eax" => self.rax.eax = value as u32,
            "ax" => self.rax.ax = value as u16,
            "ah" => self.rax.inner.ah = value as u8,
            "al" => self.rax.inner.al = value as u8,
            _ => panic!("bad reg name"),
        }
    }

    
}
enum OD<'a> {
    REG64(&'a u64),
    REG32(Rc<u32>),
    REG16(Rc<u16>),
    REG8(Rc<u8>),
}

fn change_reg(src: OD, dst: OD) {
    match src {
        OD::REG64(value) => {
            match dst {
                OD::REG64(dst_value) => {
                    // *dst_value.borrow_mut() = *(*value).borrow();
                    // println!("zzzz{:x}",*(*dst_value).borrow());
                }
                OD::REG32(_) => todo!(),
                OD::REG16(_) => {
                    todo!();
                }
                OD::REG8(_) => todo!(),
            }
        }
        OD::REG32(_) => todo!(),
        OD::REG16(_) => todo!(),
        OD::REG8(_) => todo!(),
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn do_work() {
        let mut core = Core::new();
        let x = 0x0000_ffff_0000_f0f2u64;
        core.update_reg("rax", x);
        unsafe {
            assert_eq!(core.rax.rax, 0x0000_ffff_0000_f0f2u64);
            assert_eq!(core.rax.eax, 0x0000_f0f2u32);
            assert_eq!(core.rax.ax,  0xf0f2u16);
            assert_eq!(core.rax.inner.ah, 0xf0u8);
            assert_eq!(core.rax.inner.al, 0xf2u8);
        }
    }
}
