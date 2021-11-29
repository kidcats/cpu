
pub struct  pa_address {
    address_value : u64
}

impl pa_address {

    pub fn new(value : u64) -> Self{
        pa_address { address_value: value }
    }
    pub fn paddr_value(&self) -> u64{
        return self.address_value & 0xfffffffffffff ;
    }
    pub fn ppo(&self) -> usize{
        return (self.address_value & 0xfff) as usize;
    }
    pub fn ppn(&self) -> usize{
        return ((self.address_value & 0xffffffffff000) >> 12) as usize;
    }
    pub fn co(&self) -> usize{
        return (self.address_value & 0x3f) as usize;
    }
    pub fn ci(&self) -> usize{
        return ((self.address_value & 0xfc0) >> 6) as usize;
    }
    pub fn ct(&self) -> usize{
        return ((self.address_value & 0xffffffffff000) >> 12) as usize;
    }
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn do_test(){
        println!("hello world");
        let _c = pa_address::new(0b10000000000001);
        assert_eq!(_c.address_value,0b10000000000001);
        // assert_eq!(_c.ppo(),0xbcd);
        // assert_eq!(_c.ppn(),0xfabcdffffa);
        assert_eq!(_c.co(),0x1);
        assert_eq!(_c.ci(),0x0);
        assert_eq!(_c.ct(),0x2);
    }
}
// 000000000000_1111111111111111111111111111111111111111_000000_000000
// 111111111111_1111101010111100110111111111111111111010_101111_001101
//              1111101010111100110111111111111111111010_000000_000000