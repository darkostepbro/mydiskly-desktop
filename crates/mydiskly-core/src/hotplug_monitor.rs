use std::sync::atomic::{AtomicU32, Ordering};
use windows_sys::Win32::Storage::FileSystem::GetLogicalDrives;

pub static LAST_DRIVE_MASK: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HotplugEvent {
    None,
    DriveConnected(char),
    DriveDisconnected(char),
}

pub fn check_hotplug_change() -> HotplugEvent {
    let current_mask = unsafe { GetLogicalDrives() };
    let previous_mask = LAST_DRIVE_MASK.swap(current_mask, Ordering::SeqCst);

    if previous_mask == 0 {
        return HotplugEvent::None;
    }

    if current_mask > previous_mask {
        let diff = current_mask & !previous_mask;
        for i in 0..26 {
            if (diff & (1 << i)) != 0 {
                let letter = (b'A' + i as u8) as char;
                return HotplugEvent::DriveConnected(letter);
            }
        }
    } else if current_mask < previous_mask {
        let diff = previous_mask & !current_mask;
        for i in 0..26 {
            if (diff & (1 << i)) != 0 {
                let letter = (b'A' + i as u8) as char;
                return HotplugEvent::DriveDisconnected(letter);
            }
        }
    }

    HotplugEvent::None
}
