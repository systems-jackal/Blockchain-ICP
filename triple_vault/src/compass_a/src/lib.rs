use ic_cdk::{query, update};
use std::cell::RefCell;
use std::collections::BTreeMap;

thread_local! {
    static STORE: RefCell<BTreeMap<String, String>> = RefCell::new(BTreeMap::new());
}

#[update]
fn put(key: String, value: String) {
    STORE.with(|store| {
        store.borrow_mut().insert(key, value);
    });
}

#[query]
fn get(key: String) -> Option<String> {
    STORE.with(|store| store.borrow().get(&key).cloned())
}

#[query]
fn get_all() -> Vec<(String, String)> {
    STORE.with(|store| {
        store
            .borrow()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    })
}

ic_cdk::export_candid!();
