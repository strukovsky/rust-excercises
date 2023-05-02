use std::{cell::{RefCell}, rc::Rc};

pub enum Link<T>  {
    None,
    Node(Box<LinkedNode<T>>),
}

pub struct LinkedNode<T> {
    pub data: Rc<RefCell<T>>,
    pub next: Link<T>
}

impl <T> LinkedNode<T> {
    
    pub fn new(data: T) -> Self{
        Self { data: Rc::new(RefCell::new(data)), next: Link::None }
    }

    pub fn append(&mut self, data: T) {
        let item = Box::new(Self::new(data));
        match self.next {
            Link::Node(_) => {panic!("Attempt to insert the middle");} 
            Link::None => {
                self.next = Link::Node(item);
            }
        }
    }
}

pub struct LinkedList<T> {
    pub head: Option<Box<LinkedNode<T>>>,
    pub size: usize,
}

impl <T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, size: 0 }
    }

    pub fn append(&mut self, data: T) {
        let mut item = Box::new(LinkedNode::new(data));
        match self.head.take() {
            Some(x) => {
                item.next = Link::Node(x);
                self.head = Some(item);
            }
            None => {
                self.head = Some(item);
            }
        }
    }

    pub fn get(&self, index: usize) -> Rc<RefCell<T>>{
        match &self.head {
            Some(x) => {
                self.__get(index, 0,x.as_ref())
            }
            None => {panic!("Cannot iterate through non-initialized list");}
        }
    }

    fn __get(&self, index: usize, iter: usize, item: &LinkedNode<T>) -> Rc<RefCell<T>> {
        if index == iter {
            return item.data.clone()
        } else {
            match &item.next {
                Link::None => {
                    panic!("Out of bounds");
                }
                Link::Node(x) => {
                    self.__get(index, iter+1, x)
                }
            }
        }
    }
}
