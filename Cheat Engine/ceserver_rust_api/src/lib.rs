pub mod api;
pub mod mem_tools;
pub mod errors;
use crate::api::open_process;

#[cfg(test)]
mod tests {
    use crate::api::{open_process, close_handle, init_api, read_process_memory_i32, read_process_memory_u64};
    use crate::mem_tools::find_base_address;

    #[test]
    fn test() {
        let pid = 10177;
        let base = find_base_address(pid, "nvidia0");
        if base.is_ok() {
            let base = base.unwrap_or(0);
            println!("Base: {:#x}", base);
        } else {
            eprintln!("Error: {}", base.unwrap_err().0)
        }
    }
}
