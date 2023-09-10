
use std::sync::Arc;

struct myrc<T>{
    src: Arc<T>
}
impl<T> myrc<T> {
    fn new(p:T)->Self{
        myrc { src: (Arc::new(p)) }
    }
    fn strong_count(&self)->usize{
        Arc::strong_count(&self.src)
    }
}
impl<T> Clone for myrc<T>{
    fn clone(&self) -> myrc<T> {
        myrc { src: Arc::clone(&self.src) }
    }
}
fn main(){
    let five = myrc::new(5);
    let five1 = five.clone();
    println!("{}",myrc::strong_count(&five1));
}

