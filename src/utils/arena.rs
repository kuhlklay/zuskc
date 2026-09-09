use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone)]
pub struct Arena<K: From<usize>, V> {
    _marker: PhantomData<K>,
    values: Vec<V>,
}

#[allow(dead_code)]
impl<K, V> Arena<K, V>
where
    K: From<usize> + Into<usize>,
    V: PartialEq,
{
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub const fn len(&self) -> usize {
        self.values.len()
    }

    pub fn push(&mut self, value: V) -> K {
        let id = K::from(self.values.len());

        self.values.push(value);

        id
    }

    pub fn get(&self, key: K) -> Option<&V> {
        self.values.get(key.into())
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.values.get_mut(key.into())
    }

    pub fn contains(&self, value: V) -> bool {
        self.values.contains(&value)
    }
}

#[allow(dead_code)]
impl<K, V> Index<usize> for Arena<K, V>
where
    K: From<usize>,
{
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<K, V> IndexMut<usize> for Arena<K, V>
where
    K: From<usize>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}
