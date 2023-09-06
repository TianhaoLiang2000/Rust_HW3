use std::cell::RefCell;

pub struct MyRc<T: Clone> {
    pub data: RefCell<T>,
    pub count: RefCell<usize>,
}

impl<T: Clone> MyRc<T> {
    pub fn new(data: T) -> Self {
        MyRc {
            data: RefCell::new(data),
            count: RefCell::new(1),
        }
    }

    pub fn clone(&self) -> Self {
        let new_count = *self.count.borrow() + 1;
        *self.count.borrow_mut() = new_count;
        MyRc {
            data: self.data.clone(),
            count: self.count.clone(),
        }
    }
}

impl<T: Clone> std::ops::Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data.as_ptr() }
    }
}

impl<T: Clone> Drop for MyRc<T> {
    fn drop(&mut self) {
        let new_count = *self.count.borrow() - 1;
        *self.count.borrow_mut() = new_count;
        if new_count == 0 {
            println!("释放内存");
        }
    }
}
