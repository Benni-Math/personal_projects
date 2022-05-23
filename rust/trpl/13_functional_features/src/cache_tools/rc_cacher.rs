use std::collections::hash_map;
use std::hash::Hash;
use std::rc::Rc;

pub struct Cacher<T, K, V>
where
    T: Fn(K) -> V,
{
    calculation: T,
    values: hash_map::HashMap<K, Rc<V>>,
}

impl<T, K: Eq + Hash + Clone, V> Cacher<T, K, V>
where
    T: Fn(K) -> V,
{
    pub fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            values: hash_map::HashMap::new(),
        }
    }

    pub fn value(&mut self, k: K) -> Rc<V> {
        let f_ref = &self.calculation;
        self.values
            .entry(k.clone())
            .or_insert_with(|| Rc::new(f_ref(k)))
            .clone()
    }
}