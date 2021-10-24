
fn _test_box(){
    let b = Box::new(5);
    let x = 5;
    let _y = &x;
    println!("{}",b)
}


#[cfg(test)]
mod test{
    use super::_test_box;

    #[test]
    fn work(){
        _test_box()
    }
}