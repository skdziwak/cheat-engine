use crate::api::extern_api::{HANDLE, DWORD};

mod extern_api;

pub fn OpenProcess(pid: DWORD) -> HANDLE {
    unsafe {
        return extern_api::OpenProcess(pid);
    }
}

pub fn CloseHandle(handle: HANDLE) {
    unsafe {
        extern_api::CloseHandle(handle);
    }
}