#![allow(dead_code)]

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
    entry_allocation_expansion_index : u64,
    entry_allocation_table : HashMap<u64, bool>,
    entries : HashMap<u64, BufferEntry>,
}

struct Allocation {
    index : u64,
    is_new_alloc : bool
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            entry_allocation_expansion_index : 0,
            entry_allocation_table : HashMap::new(),
            entries : HashMap::new()
        }
    }

    pub fn push(&mut self, data : BufferEntry) {
        let alloc : Allocation = self.allocate_entry();

        if alloc.is_new_alloc {
            self.entry_allocation_expansion_index += 1;
        }

        self.entry_allocation_table.insert(alloc.index, true);

        self.entries.insert(alloc.index, data);
    }

    fn allocate_entry(&self) -> Allocation {
        for (index, in_use) in &self.entry_allocation_table {
            if !in_use {
                return Allocation {
                    index : *index,
                    is_new_alloc : false
                };
            }
        }

        Allocation { index: self.entry_allocation_expansion_index + 1, is_new_alloc: true }
    }

    pub fn get_addr(&self, addr : u64) -> Option<&BufferEntry> {
        self.entries.get(&addr)
    }

    pub fn set_addr_data(&mut self, addr : u64, data : Box<[u8]>) -> Result<(), ()> {
        let mut entry = match self.entries.get(&addr) {
            Some(x) => x,
            None => return Result::Err(())
        }.clone();

        entry.set_data(data);

        self.entries.insert(addr, entry);

        Ok(())
    }
}