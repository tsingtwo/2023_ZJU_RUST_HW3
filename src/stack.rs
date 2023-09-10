use std::cell::RefCell;


pub struct MyStack<T> {
    stack: RefCell<Vec<T>>,
}

impl<T> MyStack<T> {
    pub fn new() -> MyStack<T> {
        MyStack{
            stack: RefCell::new(Vec::new()),
        }
    }

    pub fn push(&self, value: T) {
        self.stack.borrow_mut().push(value)
    }

    pub fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}