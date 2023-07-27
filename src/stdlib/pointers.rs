use std::rc::Rc;

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[inline]
fn some_action_with_value(value: &i32) {
    println!("{value:?}");
}

fn iterate_list(current: Box<List>) {
    match *current {
        List::Cons(value, next) => {
            some_action_with_value(&value);
            iterate_list(next);
        },
        List::Nil => {
            println!("End")
        }
    }
}

pub fn simple_demo_box() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    iterate_list(Box::new(list));
}


pub enum Tree {
    Item(i32, Rc<Tree>),
    Nil
}

fn iterate_tree(root: Rc<Tree>) {
    match &*root {
        Tree::Item(value, next) => {
            some_action_with_value(value);
            iterate_tree(Rc::clone(next));
        }
        Tree::Nil => {

        }
    }
}

pub fn simple_demo_rc() {
    let tree_root = Rc::new(Tree::Item(10, Rc::new(Tree::Nil)));
    let left_branch = Rc::new(Tree::Item(3, Rc::new(Tree::Item(7, Rc::clone(&tree_root)))));
    let right_branch = Rc::new(Tree::Item(20, Rc::new(Tree::Item(40, Rc::clone(&tree_root)))));
    println!("From left branch");
    iterate_tree(Rc::clone(&left_branch));
    println!("From root");
    iterate_tree(Rc::clone(&tree_root));
    println!("From right branch");
    iterate_tree(Rc::clone(&right_branch));
}
