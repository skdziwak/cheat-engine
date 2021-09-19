use crate::api::extern_api::{HANDLE, DWORD, POINTER};

pub mod extern_api;

pub fn open_process(pid: &DWORD) -> HANDLE {
    unsafe {
        return extern_api::OpenProcess(pid.clone());
    }
}

pub fn close_handle(handle: HANDLE) {
    unsafe {
        extern_api::CloseHandle(handle);
    }
}

pub fn read_process_memory_i32(handle: &HANDLE, address: &u64) -> i32 {
    unsafe {
        let mut result: i32 = 0;
        extern_api::ReadProcessMemory(handle.clone(), address.clone() as POINTER, &mut result, 4);
        return result;
    }
}

pub fn read_process_memory_u64(handle: &HANDLE, address: &u64) -> u64 {
    unsafe {
        let mut result: u64 = 0;
        let ptr: *mut u64 = &mut result;
        extern_api::ReadProcessMemory(handle.clone(), address.clone() as POINTER, ptr as POINTER, 8);
        return result;
    }
}

pub fn init_api() {
    unsafe {
        extern_api::initAPI();
    }
}