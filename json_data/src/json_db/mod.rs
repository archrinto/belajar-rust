use std::collections::HashMap;

pub mod participant;
pub mod categories;

pub trait Database<T> {
    fn new() -> Self;
    fn all(&self) -> Vec<T>;
    fn load(&mut self);
    fn save(&self);
    fn indexing(&mut self);
    fn add(&mut self, item: &T);
    fn bluk_add(&mut self, items: &Vec<T>);
    fn clear(&mut self);
    fn get_by_index(&self, key: &str, value: &str) -> Option<T>;
    fn update_by_index(&mut self, key: &str, value: &str, obj: T);
    fn remove_by_index(&mut self, key: &str, value: &str);
    fn get_index(&self, key: &str, value: &str) -> Option<&usize>;
}