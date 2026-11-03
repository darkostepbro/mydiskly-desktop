use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS};
use windows_sys::Win32::System::Threading::CreateMutexW;

use crate::gatekeeper::to_wide_chars;

pub const SINGLE_INSTANCE_MUTEX_NAME: &str = r"Local\MyDiskly_SingleInstance_Mutex_Darkostepbro";

pub struct SingleInstanceGuard {
    handle: isize,
}

impl Drop for SingleInstanceGuard {
    fn drop(&mut self) {
        if self.handle != 0 {
            unsafe {
                CloseHandle(self.handle);
            }
        }
    }
}

pub fn try_acquire_single_instance() -> Option<SingleInstanceGuard> {
    let name = to_wide_chars(SINGLE_INSTANCE_MUTEX_NAME);
    let handle = unsafe { CreateMutexW(std::ptr::null(), 1, name.as_ptr()) };

    if handle == 0 {
        return None;
    }

    let err = unsafe { GetLastError() };
    if err == ERROR_ALREADY_EXISTS {
        unsafe {
            CloseHandle(handle);
        }
        return None;
    }

    Some(SingleInstanceGuard { handle })
}
