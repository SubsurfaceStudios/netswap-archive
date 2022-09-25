use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub mod permissions;

#[derive(Debug, Clone)]
pub struct BufferEntry {
    pub owner_hash : u64,
    pub data_reference : Box<[u8]>,
    pub non_owner_permissions : u8
}

impl BufferEntry {
    pub fn new(owner_ip : String, non_owner_permissions : u8) -> Self {
        let mut hasher = DefaultHasher::new();
        owner_ip.hash(&mut hasher);

        Self {
            owner_hash: hasher.finish(),
            data_reference: Box::new([0 as u8; 0]),
            non_owner_permissions
        }
    }

    pub fn set_data(&mut self, data : Box<[u8]>) {
        self.data_reference = data;
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data_reference
    }
}


pub struct Buffer {
    pub entry_count : u128,
    entries : HashMap<u128, BufferEntry>
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            entry_count : 0,
            entries : HashMap::new()
        }
    }

    pub fn push(&mut self, data : BufferEntry) {
        self.entries.insert(self.entry_count, data);

        self.entry_count += 1;
    }

    pub fn get_addr(&self, addr : u128) -> Option<&BufferEntry> {
        self.entries.get(&addr)
    }

    pub fn set_addr_data(&mut self, addr : u128, data : Box<[u8]>) -> Result<(), ()> {
        let mut entry = match self.entries.get(&addr) {
            Some(x) => x,
            None => return Result::Err(())
        }.clone();

        entry.set_data(data);

        self.entries.insert(addr, entry);

        Ok(())
    }
}