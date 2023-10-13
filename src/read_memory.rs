use winapi;
use std;
use sysinfo;
use winapi::ctypes::c_void;
use winapi::shared::ntdef::HANDLE;
use winapi::shared::minwindef::LPCVOID;
use winapi::{um::winnt::PROCESS_ALL_ACCESS, shared::minwindef::HINSTANCE__};

pub fn read_memory(hprocess: HANDLE, baseaddress: LPCVOID, print_buffer: bool) -> [u8; 10]
{
unsafe{
    let mut auto_buffer: [u8; 10] = [0; 10];

    winapi::um::memoryapi::ReadProcessMemory(hprocess,
        baseaddress as *mut c_void,
        auto_buffer.as_mut_ptr().cast(),
       10, 
       &mut 0);
    
    if print_buffer == true {
    println!("[READ_MEMORY->AUTO_BUFFER]: {:?}", auto_buffer);
    return auto_buffer;
    }
    else {
        return auto_buffer;
    }
    }
}

pub fn write_memory(hprocess: HANDLE, baseaddress: LPCVOID, new_amount: u64, print_buffer: bool) -> [u64; 1]
{
    unsafe {
        let mut auto_buffer: [u64; 1] = [new_amount];
        
        winapi::um::memoryapi::WriteProcessMemory(hprocess,
        baseaddress as *mut c_void,
        auto_buffer.as_mut_ptr().cast(),
        4,
        &mut 0);
    if print_buffer == true{
    println!("[WRITE_MEMORY->AUTO_BUFFER]: {:?}", auto_buffer);
    return auto_buffer;
    } else {
        return auto_buffer;
    }
    }
}