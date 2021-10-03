
use crate::hardware::memory::dram::PHYSICAL_MEMORY_SPACE;

pub fn va2pa(pa:u64)->Option<u64>{
    return Some(pa % PHYSICAL_MEMORY_SPACE as u64);
}