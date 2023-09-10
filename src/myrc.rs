use std::ptr;
use std::{sync::Arc, ptr::NonNull, mem};
use std::ops::Deref;

pub struct Myrc<T>{
    src: Arc<T>
}
impl<T> Myrc<T> {
    pub fn new(p:T)->Self{
        Myrc { src: (Arc::new(p)) }
    }
    pub fn strong_count(&self)->usize{
        Arc::strong_count(&self.src)
    }
}
impl<T> Clone for Myrc<T>{
    fn clone(&self) -> Myrc<T> {
        Myrc { src: Arc::clone(&self.src) }
    }
}

pub struct Rc_<T>{
    val: T,
    cnt: i32,
}
#[allow(unused)]
impl<T> Rc_<T> {
    pub fn new(val: T)->Rc_<T>{
        Rc_ { val: val, cnt: 1 }
    }
    // pub fn cnt_inc(&mut self)->i32{
    //     self.cnt += 1;
    //     self.cnt
    // }
    // pub fn cnt_dec(&mut self)->i32{
    //     self.cnt -= 1;
    //     self.cnt
    // }
    // 上面的函数发现后面根本用不了，暂时没搞明白
    pub fn get_cnt(&mut self) -> i32{
        self.cnt
    }
}
pub struct Myrc1<T>{
    rc: NonNull<Rc_<T>>,
}
#[allow(unused)]
impl<T> Myrc1<T> {
    pub fn new(val: T)->Myrc1<T>{
        let mut rcptr = Box::new(Rc_::new(val));
        let _rc = Myrc1{rc: NonNull::new(&mut *rcptr).unwrap()};
        mem::forget(rcptr);
        _rc
    }
    pub fn strong_count(&self) -> i32 {
        unsafe { (*(NonNull::<Rc_<T>>::as_ptr(self.rc))).get_cnt() }
    }
    pub fn clone(&mut self) -> Myrc1<T> {
        // (unsafe { *(NonNull::<rc<T>>::as_ptr(self.rc)) }).cnt_inc();
        // 上面这句取不出来，也没整出啥，遂放弃
        let tmp = unsafe { self.rc.as_ref().cnt } + 1;
        unsafe { self.rc.as_mut().cnt = tmp };
        Myrc1{rc: self.rc}
    }
    pub fn drop(&mut self) {
        unsafe {
            let tmp = self.rc.as_ref().cnt - 1;
            self.rc.as_mut().cnt = tmp;
            if tmp == 0 {
                mem::drop(ptr::read(self.rc.as_ptr()));
            }
        }
}
}
impl<T> Deref for Myrc1<T> {
    type Target = T;
     fn deref(&self) -> &Self::Target {
        unsafe {
            &self.rc.as_ref().val
        }
    }
}
impl<T> Deref for Myrc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.src.deref()
    }
}