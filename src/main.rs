use crate::myrc::Myrc;

mod myrc;
mod hashmap;
mod stack;
fn main(){

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hash_map() {
        let  mymap:HashMap<i32,&str> =  my_hash_map!(1 => "嗯", 2 => "嘛", 3=> "啊");
        assert_eq!(mymap.get(&1),Some(&"嗯"));
        assert_eq!(mymap.get(&2),Some(&"嘛"));
        assert_eq!(mymap.get(&3),Some(&"啊"));
    }

    #[test]
    fn test_stack() {
        let stack = MyStack::new();
        stack.push(19);
        stack.push(19);
        stack.push(810);
        assert_eq!(stack.pop(),Some(810));
        assert_eq!(stack.pop(),Some(19));
        assert_eq!(stack.pop(),Some(19));
        assert_eq!(stack.pop(),None);
    }
    use std::collections::HashMap;

    use crate::{myrc::Myrc1, stack::MyStack};

    use super::*;

    #[test]
    fn test_rc_from_arc() {
        let s = Myrc::new("123");
        let s1 = s.clone();

        assert_eq!(Myrc::strong_count(&s1), 2);
        assert_eq!(*s1, "123");

        {
            let s2 = s1.clone();
            assert_eq!(Myrc::strong_count(&s1), 3);
            assert_eq!(*s2, "123");
        }
        assert_eq!(Myrc::strong_count(&s1), 2);
        assert_eq!(*s, "123");
    }
    #[test]
    fn test_myrc() {
        let mut _rc1 = Myrc1::new(114514);
        assert_eq!(*_rc1,114514);
        assert_eq!(_rc1.strong_count(),1);

        let mut _rc2 = _rc1.clone();
        assert_eq!(*_rc2,114514);
        assert_eq!(_rc1.strong_count(),2);
        assert_eq!(_rc2.strong_count(),2);
    }
    
}

