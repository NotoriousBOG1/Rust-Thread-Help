use std::io;
use read_process_memory::*;

fn main() {
    read_some_memory(4848 as Pid, 0x7ff74c4063f0, 4).unwrap();
}

fn read_some_memory(pid: Pid, address: usize, size: usize) -> io::Result<()> {
    let handle = pid.try_into_process_handle()?;
    let bytes = copy_address(address, size, &handle)?;
    println!("result: {:?}", bytes);
    Ok(())
}
