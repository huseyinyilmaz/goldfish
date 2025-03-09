use std::{collections::HashMap, time::SystemTime};

use log::debug;

use crate::utils;


#[derive(Debug)]
pub struct Data {
    pub data: Vec<u8>,
    pub timeout: u64,
    pub flags: i32,
    pub time: SystemTime,
}

#[derive(Debug)]
pub struct State {
    data: HashMap<Vec<u8>, Data>
}

impl State {
    pub fn new() -> Self {
        return State{data: HashMap::new()};
    }

    pub fn set_key(&mut self, key: Vec<u8>, data: Data) -> Option<Data> {
        self.data.insert(key, data)
    }

    pub fn get_key(&self, key: &Vec<u8>) -> Option<&Data> {
        debug!("keys = ");
        for key in self.data.keys() {
            debug!("key={:?}", utils::raw_string_to_string(key));
        }
        return self.data.get(key);
    }
}
