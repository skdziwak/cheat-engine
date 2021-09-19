pub type DWORD = u32;
pub type HANDLE = u32;
pub type WORD = u16;

#[link(name = "ceserver")]
extern {
    pub fn OpenProcess(pid: DWORD) -> HANDLE;
    pub fn CloseHandle(handle: HANDLE);
}