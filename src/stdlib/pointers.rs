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
