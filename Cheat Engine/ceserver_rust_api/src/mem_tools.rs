use std::fs::{File, read_to_string};
use crate::errors::Error;
use crate::api::read_process_memory_u64;
use crate::api::extern_api::HANDLE;

pub fn find_base_address<S: Into<String>>(pid: u32, name: S) -> Result<u64, Error> {
    let name = name.into();
    let path = format!("/proc/{}/maps", pid);
    let maps = read_to_string(path)?;
    for line in maps.lines() {
        let line = String::from(line);
        let line = String::from(line.trim());
        if line.ends_with(&name) {
            let index = line.find("-");
            if index.is_some() {
                let index = index.unwrap();
                let hex = &line[0..index];
                let address = u64::from_str_radix(hex, 16)?;
                return Ok(address);
            }
        }
    }

    Err(Error(format!("Memory region {} not found.", name)))
}

pub fn follow(handle: HANDLE, base: u64, offsets: Vec<u64>) -> u64 {
    let mut ptr: u64 = base;
    for offset in offsets {
        ptr = read_process_memory_u64(handle, ptr);
        ptr += offset;
    }
    return ptr;
}