use winapi::ctypes::c_void;
use winapi::shared::minwindef::{BOOL, DWORD};
use winapi::um::fileapi::WriteFile;
use winapi::um::minwinbase::LPOVERLAPPED;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::STD_OUTPUT_HANDLE;
use winapi::um::winnt::HANDLE;

use core::ptr::{null, null_mut};
use core::str;

pub fn write_stdout(text: &str) -> BOOL {
    unsafe {
        let stdout: HANDLE = GetStdHandle(STD_OUTPUT_HANDLE);

        let mut written: DWORD = 0;

        let ok = WriteFile(
            stdout,
            (text.as_ptr() as *const c_void),
            text.len() as u32,
            &mut written,
            null_mut(),
        );

        ok
    }
}
