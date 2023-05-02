use std::{cell::RefCell, fmt::Display, rc::Rc, borrow::Borrow};

pub struct DequeueNode<T> {
    pub data: Rc<RefCell<T>>,
    pub next: Rc<RefCell<Option<DequeueNode<T>>>>,
    pub prev: Rc<RefCell<Option<DequeueNode<T>>>>,
}

impl<T> DequeueNode<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Rc::new(RefCell::new(data)),
            next: Rc::new(RefCell::new(None)),
            prev: Rc::new(RefCell::new(None)),
        }
    }
}

pub struct Dequeue<T> {
    pub head: Rc<RefCell<Option<DequeueNode<T>>>>,
    pub tail: Rc<RefCell<Option<DequeueNode<T>>>>,
}

impl<T> Dequeue<T> {
    pub fn new() -> Self {
        Dequeue {
            head: Rc::new(RefCell::new(None)),
            tail: Rc::new(RefCell::new(None)),
        }
    }

    pub fn push_front(&mut self, data: T)
    where
        T: Display,
    {
        println!("Adding {}", data);
        let mut new_element = DequeueNode::new(data);
        match self.head.take() {
            None => {
                self.head = Rc::new(RefCell::new(Some(new_element)));
                self.tail = self.head.clone();
            }
            Some(current_head) => {
                new_element.next = Rc::new(RefCell::new(Some(current_head)));
                self.head = Rc::new(RefCell::new(Some(new_element)));
            }
        }
    }

    pub fn get_from_front(&self, index: usize) -> Rc<RefCell<T>> {
        self.__get_from_front(index, 0, self.head.clone())
    }

    fn __get_from_front(
        &self,
        index: usize,
        iter: usize,
        current_item: Rc<RefCell<Option<DequeueNode<T>>>>,
    ) -> Rc<RefCell<T>> {
        println!("{}", iter);
        if let Some(current_item_unwrapped) = current_item.take() {
            if iter == index {
                return current_item_unwrapped.data.clone();
            } else {
                return self.__get_from_front(index, iter + 1, current_item_unwrapped.next.clone());
            }
        } else {
            println!("{} Zero at index", iter);
            panic!("Out of bounds");
        }
    }
}
