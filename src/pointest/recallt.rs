use std::{cell::RefCell, rc::Rc};


pub trait Messenger {
    fn send(&self, msg: &str);
}


#[warn(dead_code)]
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}
#[warn(dead_code)]
impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
             self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}



#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>,Rc<List>),
    Nil,
}



#[cfg(test)]
mod test{
    use std::{cell::RefCell, vec};

    // use super::_test_box;
    use super::*;

    struct MockMessenger{
        messengers : RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> Self { 
            MockMessenger{
                messengers : RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger{
        fn send(&self, msg: &str) {
            self.messengers.borrow_mut().push(String::from(msg));
        }
    }


    #[test]
    fn work(){
        let mock = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mock, 100);
        tracker.set_value(80);
        assert_eq!(mock.messengers.borrow().len(),1);
    }

    #[test]
    fn rc_with_refcell(){
        use super::List::{Cons,Nil};
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons(Rc::clone(&value),Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(6)),Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)),Rc::clone(&a));
        *value.borrow_mut() += 10;
        println!("a : {:?}",&a);
        println!("b : {:?}",&b);
        println!("c : {:?}",&c);
    }
}