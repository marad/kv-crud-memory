#[macro_use]
extern crate failure;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::hash::Hash;

use kv_crud_core::*;

#[derive(Debug, Fail)]
pub enum InMemoryStorageError {
    #[fail(display = "Entity with id {} was not found", entity_id)]
    EntityNotFound { entity_id: String },
}

/// In-memory storage manages it's data in simple hash map
#[derive(Default)]
pub struct InMemoryStorage<K, V>
where
    K: Hash + Eq,
    V: Entity<K>,
{
    db: HashMap<K, V>,
}

impl<K, V> InMemoryStorage<K, V>
where
    K: Hash + Eq,
    V: Entity<K>,
{
    /// Creates new empty in-memory storage
    pub fn new() -> Self {
        Self { db: HashMap::new() }
    }
}

impl<K, V> Create<K, V> for InMemoryStorage<K, V>
where
    K: Hash + Eq,
    V: Entity<K>,
{
    type Error = InMemoryStorageError;
    fn save(&mut self, entity: &V) -> Result<(), InMemoryStorageError> {
        self.db.insert(entity.get_id(), entity.clone());
        Ok(())
    }
}

impl<K, V> Read<K, V> for InMemoryStorage<K, V>
where
    K: Hash + Eq + ToString,
    V: Entity<K>,
{
    type Error = InMemoryStorageError;

    fn find_by_id(&self, id: &K) -> Result<V, InMemoryStorageError> {
        match self.db.get(id) {
            Some(value) => Ok(value.clone()),
            None => {
                let id: String = id.to_string();
                Err(InMemoryStorageError::EntityNotFound { entity_id: id })
            }
        }
    }
}

impl<K, V> ReadWithPaginationAndSort<K, V> for InMemoryStorage<K, V>
where
    K: Hash + Eq + ToString,
    V: Entity<K> + Ord,
{
    type Error = InMemoryStorageError;

    fn find_all_with_page(&self, page: &Page) -> Result<Vec<V>, InMemoryStorageError> {
        self.find_all_with_page_and_sort(page, &Sort::ASCENDING)
    }

    fn find_all_with_page_and_sort(
        &self,
        page: &Page,
        sort: &Sort,
    ) -> Result<Vec<V>, InMemoryStorageError> {
        let mut values: Vec<V> = self.db.iter().map(|(_, v)| v).cloned().collect();

        values.sort();
        if *sort == Sort::DESCENDING {
            values.reverse();
        }

        Ok(values
            .into_iter()
            .skip(page.offset() as usize)
            .take(page.size as usize)
            .collect())
    }
}

impl<K, V> Update<K, V> for InMemoryStorage<K, V>
where
    K: Hash + Eq,
    V: Entity<K>,
{
    type Error = InMemoryStorageError;

    fn update(&mut self, entity: &V) -> Result<(), Self::Error> {
        self.save(entity)
    }
}

impl<K, V> Delete<K, V> for InMemoryStorage<K, V>
where
    K: Hash + Eq,
    V: Entity<K>,
{
    type Error = InMemoryStorageError;

    fn remove_by_id(&mut self, id: &K) -> Result<(), Self::Error> {
        self.db.remove(id);
        Ok(())
    }

    fn remove(&mut self, entity: &V) -> Result<(), Self::Error> {
        self.remove_by_id(&entity.get_id())
    }
}

impl<K, V> Crud<K, V> for InMemoryStorage<K, V>
where
    K: Hash + Eq + ToString,
    V: Entity<K> + Ord,
{
}

impl<K, V> From<&V> for InMemoryStorage<K, V>
where
    K: Hash + Eq,
    V: Entity<K>,
{
    fn from(v: &V) -> Self {
        let mut result = Self::new();
        result.save(v).unwrap();
        result
    }
}

impl<K, V> From<&Vec<V>> for InMemoryStorage<K, V>
where
    K: Hash + Eq,
    V: Entity<K>,
{
    fn from(values: &Vec<V>) -> Self {
        let mut result = Self::new();
        for v in values {
            result.save(v).unwrap();
        }
        result
    }
}
