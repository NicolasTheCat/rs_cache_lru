use std::collections::HashMap;

pub struct Cache<V> {
    capacity: usize,
    data: HashMap<String, V>,
    order: Vec<String>,
}

impl<V> Cache<V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: HashMap::with_capacity(capacity),
            order: Vec::with_capacity(capacity),
        }
    }

    pub fn put(&mut self, key: &str, value: V) {
        if (self.data.contains_key(key)) {
            self.remove_from_order(key)
        } else if self.data.len() == self.capacity {
            let old = self.order.remove(0);
            self.data.remove(&old);
        }

        self.order.push(key.clone().to_string());
        self.data.insert(key.to_string(), value);
    }

    pub fn get(&mut self, key: &str) -> Option<&V> {
        if (self.data.contains_key(key)) {
            self.remove_from_order(key);
            self.order.push(key.to_string());
            return self.data.get(key);
        } else {
            return None;
        }
    }

    fn remove_from_order(&mut self, key: &str) {
        let position = self.order.iter().position(|x| x == &key);
        if (position.is_some()) {
            let pos = position.unwrap();
            self.order.remove(pos);
        }
    }
}
