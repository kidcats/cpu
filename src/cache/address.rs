
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
    pub fn ppo(&self) -> u64{
        return self.address_value & 0xfff;
    }
    pub fn ppn(&self) -> u64{
        return (self.address_value & 0xffffffffff000) >> 12;
    }
    pub fn co(&self) -> u64{
        return self.address_value & 0x3f;
    }
    pub fn ci(&self) -> u64{
        return (self.address_value & 0xfc0) >> 6;
    }
    pub fn ct(&self) -> u64{
        return (self.address_value & 0xffffffffff000) >> 12;
    }
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn do_test(){
        println!("hello world");
        let _c = pa_address::new(0xffffabcdffffabcd);
        assert_eq!(_c.address_value,0xffffabcdffffabcd);
        assert_eq!(_c.ppo(),0xbcd);
        assert_eq!(_c.ppn(),0xfabcdffffa);
        assert_eq!(_c.co(),0xd);
        assert_eq!(_c.ci(),0x2f);
        assert_eq!(_c.ct(),0xfabcdffffa);
    }
}
// 000000000000_1111111111111111111111111111111111111111_000000_000000
// 111111111111_1111101010111100110111111111111111111010_101111_001101
//              1111101010111100110111111111111111111010_000000_000000