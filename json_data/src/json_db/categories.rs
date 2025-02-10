use std::collections::HashMap;

use super::Database;
#[derive(Clone, Debug)]
pub struct Category {
    pub id: u32,
    pub name: String
}

pub struct CategoryDb {
    data: Vec<Category>,
    indexs: HashMap<String, usize>,
    file_path: String
}

impl Database<Category> for CategoryDb {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            indexs: HashMap::new(),
            file_path: String::from("./dbs/categories.json")
        }
    }

    fn all(&self) -> Vec<Category> {
        self.data.clone()
    }

    fn load(&mut self) {
        todo!()
    }

    fn save(&self) {
        todo!()
    }

    fn indexing(&mut self) {
        self.indexs.clear();
        for (i, c) in self.data.iter().enumerate() {
            self.indexs.insert(format!("id_{}", c.id), i);
        }
    }

    fn add(&mut self, obj: &Category) {
        self.data.push(obj.clone());
        self.indexing();
    }

    fn bluk_add(&mut self, items: &Vec<Category>) {
        for item in items.iter() {
            self.data.push(item.clone());
        }
    }

    fn clear(&mut self) {
        self.data.clear();
        self.indexs.clear();
    }

    fn get_by_index(&self, key: &str, value: &str) -> Option<Category> {
        self.get_index(key, value).and_then(|&index| self.data.get(index as usize).cloned())
    }

    fn update_by_index(&mut self, key: &str, value: &str, obj: Category) {
        if let Some(&index) = self.get_index(key, value) {
            self.data[index as usize] = obj;
            self.indexing();
        }
    }

    fn remove_by_index(&mut self, key: &str, value: &str) {
        if let Some(&index) = self.get_index(key, value) {
            self.data.remove(index as usize);
            self.indexing();
        }
    }

    fn get_index(&self, key: &str, value: &str) -> Option<&usize> {
       self.indexs.get(format!("{}_{}", key, value).as_str())
    }
}

