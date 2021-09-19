pub mod api;
pub mod mem_tools;
pub mod errors;
use crate::api::open_process;

#[cfg(test)]
mod tests {
    use crate::api::{open_process, close_handle, init_api, read_process_memory_i32, read_process_memory_u64};
    use crate::mem_tools::{find_base_address, follow};
    use crate::api::extern_api::OpenProcess;

    #[test]
    fn test() {
        let pid = 34390;
        let handle = open_process(34390);
        let base = find_base_address(pid, "libhl.so");
        let offset: u64 = 0x0027C780;
        let offsets: Vec<u64> = vec![0x860, 0x0, 0x68, 0x80, 0xE8, 0x8, 0x48];
        if base.is_ok() {
            let base = base.unwrap_or(0);
            //println!("base + offset: {:#x}", base + offset);
            let newbase = base + offset;

            let gold_p: u64 = follow(&handle, &newbase, &offsets);
            let gold = read_process_memory_i32(handle, gold_p);
            println!("Gold: {}", gold);
        } else {
            eprintln!("Error: {}", base.unwrap_err().0)
        }
    }
}
