use std::collections::HashMap;
use crate::State;
use std::fmt;

#[derive(Clone)]
pub struct MemoryLayout {
    ports: HashMap<u16, fn(&mut State, u16, u8)>,
    banks: Vec<MemoryBank>,
}

impl fmt::Debug for MemoryLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.debug_struct("MemoryLayout")
            .field("ports", &Some(()))
            .field("banks", &self.banks)
            .finish()
    }
}

impl Default for MemoryLayout {
    fn default() -> Self {
        Self {
            ports: HashMap::new(),
            banks: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryBank {
    size: usize,
    location: u16,
    writable: bool,
}

impl MemoryBank {
    fn new(size: usize, location: u16, writable: bool) -> Self {
        Self {
            size,
            location,
            writable,
        }
    }
}
