use super::*;

#[derive(Clone, Debug, PartialEq, Ord, PartialOrd, Eq)]
struct MyData {
    pub id: u32,
    pub data: u32,
}

impl Entity<u32> for MyData {
    fn get_id(&self) -> u32 {
        self.id
    }
}

#[test]
fn should_save_entity_in_storage() {
    // given
    let mut storage = InMemoryStorage::new();
    let data = MyData { id: 8, data: 42 };

    // when
    storage.save(&data).unwrap();

    // then
    assert_eq!(&data, storage.db.get(&8).unwrap());
}

#[test]
fn should_find_by_id() {
    // given
    let data = MyData { id: 42, data: 42 };
    let storage =
        InMemoryStorage::from(&vec![data.clone()]);

    // when
    let result = storage.find_by_id(&42u32).unwrap();

    // then
    assert_eq!(data, result);
}

#[test]
fn should_return_not_found() {
    // given
    let storage: InMemoryStorage<u32, MyData> = InMemoryStorage::new();

    // when
    let result = storage.find_by_id(&123u32);

    // then
    match result {
        Err(InMemoryStorageError::EntityNotFound { entity_id: id }) =>
            assert_eq!(id, "123".to_owned()),
        _ => assert!(false),
    }
}

#[test]
fn should_find_and_paginate() {
    // given
    let data = vec![
        MyData { id: 1, data: 42 },
        MyData { id: 2, data: 42 },
        MyData { id: 3, data: 42 },
    ];
    let storage = InMemoryStorage::from(&data);

    // when
    let result = dbg!(storage
        .find_all_with_page_and_sort(&Page::new(0, 1), &Sort::DESCENDING)
        .unwrap());

    // then
    assert_eq!(1, result.len());
    assert_eq!(3u32, result.first().unwrap().id);
}

#[test]
fn should_update_entity() {
    // given
    let mut storage: InMemoryStorage<u32, MyData> =
        InMemoryStorage::from(&vec![MyData { id: 1, data: 42 }]);

    let updated = MyData { id: 1, data: 24 };

    // when
    storage.update(&updated).unwrap();

    // then
    assert_eq!(&updated, storage.db.get(&1u32).unwrap());
}


#[test]
fn should_remove_entity_by_id() {
    // given
    let mut storage = InMemoryStorage::from(&MyData {id: 1, data: 42 });

    // when
    storage.remove_by_id(&1).unwrap();

    // then
    assert!(storage.db.is_empty());
}

#[test]
fn should_remove_entity() {
    let entity = MyData {id: 1, data: 42 };
    let mut storage = InMemoryStorage::from(&entity);

    // when
    storage.remove(&entity).unwrap();

    // then
    assert!(storage.db.is_empty());
}