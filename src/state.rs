use std::{
    collections::HashMap,
    sync::atomic::{AtomicU64, Ordering},
    time::SystemTime,
};

use log::debug;

use crate::utils;

#[derive(Debug)]
pub struct Data {
    pub data: Vec<u8>,
    pub timeout: i64,
    pub flags: i32,
    pub time: SystemTime,
    pub cas_unique: u64,
}

#[derive(Debug)]
pub struct State {
    data: HashMap<Vec<u8>, Data>,
    pub start_time: SystemTime,
    pub cmd_get: AtomicU64,
    pub cmd_set: AtomicU64,
    pub total_items: AtomicU64,
    pub get_hits: AtomicU64,
    pub get_misses: AtomicU64,
    next_cas: u64,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        State {
            data: HashMap::new(),
            start_time: SystemTime::now(),
            cmd_get: AtomicU64::new(0),
            cmd_set: AtomicU64::new(0),
            total_items: AtomicU64::new(0),
            get_hits: AtomicU64::new(0),
            get_misses: AtomicU64::new(0),
            next_cas: 1,
        }
    }

    pub fn set_key(&mut self, key: Vec<u8>, mut data: Data) -> Option<Data> {
        data.cas_unique = self.next_cas;
        self.next_cas += 1;
        self.data.insert(key, data)
    }

    pub fn get_key(&self, key: &Vec<u8>) -> Option<&Data> {
        debug!("keys = ");
        for key in self.data.keys() {
            debug!("key={:?}", utils::raw_string_to_string(key));
        }
        self.data.get(key)
    }

    pub fn delete_key(&mut self, key: &[u8]) -> bool {
        self.data.remove(key).is_some()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn total_bytes(&self) -> usize {
        self.data.values().map(|d| d.data.len()).sum()
    }

    pub fn increment_total_items(&self) {
        self.total_items.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_cas(&self, key: &[u8]) -> Option<u64> {
        self.data.get(key).map(|d| d.cas_unique)
    }
}
