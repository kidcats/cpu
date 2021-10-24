use std::{cell::RefCell, rc::Rc};

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
    rax: Rc<u64>,
    eax: Rc<u32>,
    ax: Rc<u16>,
    al: Rc<u8>,
    ah: Rc<u8>,
}

impl Core {
    fn new() -> Core{
        let x = RAX_REG {
            rax: 0x0000_0000_0000_0000,
        };
        unsafe{
            Core{
                rax:Rc::new(x.rax),
                eax:Rc::new(x.eax),
                ax:Rc::new(x.ax),
                al:Rc::new(x.inner.al),
                ah:Rc::new(x.inner.ah)
            }
        }
        
    }
}
enum OD{
    REG64(RefCell<u64>),
    REG32(Rc<u32>),
    REG16(Rc<u16>),
    REG8(Rc<u8>),
}


fn change_reg(src:OD,dst:OD){
    match src {
        OD::REG64(value) => {
            match dst{
                OD::REG64(dst_value) => {
                    dst_value = Rc::clone(&value);
                },
                OD::REG32(_) => todo!(),
                OD::REG16(_) => {
                    todo!();
                },
                OD::REG8(_) => todo!(),
            }
        },
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
        let core = Core::new();
        change_reg(OD::REG64(core.rax), OD::REG64(Rc::new(0x1111_1111_1111_1111)));
        println!("core {:?}",core.eax);
    }
}
