use std::{cell::RefCell, rc::Rc};

pub struct Tree {
    left: Rc<RefCell<Option<Tree>>>,
    right: Rc<RefCell<Option<Tree>>>,
    data: i32,
}

pub struct AlreadyInTreeError {}

impl Tree {
    pub fn new(data: i32) -> Self {
        Self{left: Rc::new(RefCell::new(None)), right: Rc::new(RefCell::new(None)), data}
    }

    pub fn push(&self, data: i32) -> Result<i32, AlreadyInTreeError> {
        if self.data < data {
            let mut element = self.right.borrow_mut();
            if let Some(right_node) = &mut *element {
                right_node.push(data);
            } else {
                let item = Self::new(data);
                *element = Some(item);
            }
        } else if self.data > data {
            let mut element = self.left.borrow_mut();
            if let Some(left_node) = &mut *element {
                left_node.push(data);
            } else {
                let item = Self::new(data);
                *element = Some(item);
            }
        } else {
            Err::<i32, AlreadyInTreeError>(AlreadyInTreeError{});
        }
        return Ok(data);
    }

    pub fn traverse(&self, apply_to_node_fn: fn(node: i32)) {
        if let Some(left_node ) = &*self.left.borrow(){
            left_node.traverse(apply_to_node_fn);
        }
        apply_to_node_fn(self.data);
        if let Some(right_node ) = &*self.right.borrow(){
            right_node.traverse(apply_to_node_fn);
        }
    }
}
