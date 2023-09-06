use std::cell::RefCell;

#[derive(Debug)]
pub struct SimpleStack<T> {
    pub stack: RefCell<Vec<T>>,
}

impl<T> SimpleStack<T> {
    #[allow(dead_code)]
    pub fn new() -> SimpleStack<T> {
        SimpleStack {
            stack: RefCell::new(Vec::new()),
        }
    }

    #[allow(dead_code)]
    pub fn push(&self, item: T) {
        self.stack.borrow_mut().push(item);
    }

    #[allow(dead_code)]
    pub fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}
