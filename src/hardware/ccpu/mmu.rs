use crate::{
    cache::address::{self, pa_address},
    hardware::ccpu::core::Core,
    hardware::ccpu::pte::*,
    hardware::memory::dram::PHYSICAL_MEMORY_SPACE,
};

pub fn va2pa(pa: u64) -> Option<u64> {
    return Some(pa % PHYSICAL_MEMORY_SPACE as u64);
}

/*
mmu 要做的事情就是把va_addr编程pa_addr思路如下
1.通过core的cr3来获取1级页表的基地址,
2.通过1级页表的基地址获取2级页表地址
3.通过查询页表的数值获取pa_addr的ppn
4.将ppn与va_addr的vpo合并成我们需要的pa_addr
 */

fn mmu_doing(va_addr: u64, core: &mut Core) -> pa_address {
    let va_address = pa_address::new(va_addr);
    let po = va_address.ppo() as u64;
    // 下面通过core的cr3找到对应的pte_level1,cr3是一个u64的地址,指向pte_level1的位置
    // 正常情况下是用cr的地址去找,不过我们没法这么搞,就只能用地址->对象这种方法映射一下了,用map
    let pte_level1: &pte_123 = core.l1_pte.get(&core.cr.cr3).unwrap();
    // 如果level1里面的数据合法
    if pte_level1.present() == 1 {
        // 通过pte_level1里面的数据找到pte_level2
        let pte_level2: &pte_123 = core.l2_pte.get(&pte_level1.base_addr()).unwrap();
        if pte_level2.present() == 1 {
            let pte_level3 = core.l3_pte.get(&pte_level2.base_addr()).unwrap();
            if pte_level3.present() == 1 {
                let pte_levle4 = core.l4_pte.get(&pte_level3.base_addr()).unwrap();
                if pte_levle4.present() == 1 {
                    // 到这里为止是找到了我们需要的最终pte,现在可以拿来组合成pa_addr放出去
                    let mut pa = pa_address::new(0x0);
                    pa.update_ppn(pte_levle4.base_addr());
                    pa.update_ppo(po);
                    return pa;
                } else {
                    todo!("page fault in pte_level4")
                }
            } else {
                todo!("page fault in pte_level3")
            }
        } else {
            todo!("page fault in pte_level2")
        }
    } else {
        todo!("page fault in pte_level1")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test() {}
}
