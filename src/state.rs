use std::{collections::HashMap, time::SystemTime};


#[derive(Debug)]
pub struct Data {
    pub data: Vec<u8>,
    pub timeout: i32,
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
}
