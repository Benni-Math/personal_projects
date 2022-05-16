use std::collections::HashMap;
use std::hash::Hash;

pub struct Cacher<T, K, V>
where
    T: Fn(K) -> V,
{
    calculation: T,
    values: HashMap<K, V>,
}

impl<T, K: Eq + Hash + Clone, V: Clone> Cacher<T, K, V>
where
    T: Fn(K) -> V,
{
    pub fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    pub fn value(&mut self, k: K) -> &V {
        if self.values.contains_key(&k) {
            return &self.values[&k];
        }

        self.values.insert(k.clone(), (self.calculation)(k.clone()));
        self.values.get(&k).unwrap()
    }
}