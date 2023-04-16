use std::{collections::HashMap, hash::Hash, vec::Vec, slice::Iter};

pub struct Container<T: Eq + Hash + Clone> {
    data: Vec<T>,
    size: usize,
    unique_items: HashMap<T, usize>,
    unique_items_count: usize,
}

impl <T: Eq + Hash + Clone> Container<T> {
    pub fn new() -> Self {
        Container{
            data: Vec::new(),
            size: 0,
            unique_items: HashMap::new(),
            unique_items_count: 0
        }
    }

    pub fn push(&mut self, item: T) {
        let copy_for_map = item.clone();
        self.data.push(item);
        self.size += 1;
        if let Some(entry_count) = self.unique_items.get_mut(&copy_for_map) {
            *entry_count += 1;
        } else {
            self.unique_items.insert(copy_for_map, 1);
            self.unique_items_count += 1;
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn get_unique_items_count(&self) -> usize {
        self.unique_items_count
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

}
