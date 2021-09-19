pub type DWORD = u32;
pub type HANDLE = u32;
pub type WORD = u16;
pub type POINTER = *mut i32;

#[link(name = "ceserver")]
extern {
    pub fn OpenProcess(pid: DWORD) -> HANDLE;
    pub fn CloseHandle(handle: HANDLE);
    pub fn initAPI();
    pub fn ReadProcessMemory(handle: HANDLE, address: POINTER, buffer: POINTER, size: i32);
}