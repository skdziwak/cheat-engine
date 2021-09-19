pub mod api;
use crate::api::open_process;

#[cfg(test)]
mod tests {
    use crate::api::{open_process, close_handle, init_api, read_process_memory_i32, read_process_memory_u64};

    #[test]
    fn test() {
        init_api();
        let handle = open_process(11840);

        let value = read_process_memory_u64(handle, 0x7FFDDB9D30F8);
        println!("Value: {}", value);

        close_handle(handle);
    }
}
