#![deny(missing_docs)]
//! This crate provides a single function [`started_by_explorer`]
//!
//! [`started_by_explorer`] can be used to check if a windows executable was launched
//! by double clicking on the executable or if instead in was launched from the terminal

#[cfg(target_os = "windows")]
mod launch_check {
    use std::{
        ffi::CStr,
        mem::{size_of, transmute},
        ops::Deref as _,
        process::id,
    };
    use windows::{
        core::{self, Owned},
        Win32::System::Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32,
            TH32CS_SNAPPROCESS,
        },
    };

    fn get_process_entry(pid: u32) -> core::Result<PROCESSENTRY32> {
        unsafe {
            let snapshot = Owned::new(CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, pid)?);
            let mut proc_entry = PROCESSENTRY32 {
                dwSize: size_of::<PROCESSENTRY32>() as u32,
                ..Default::default()
            };
            Process32First(*snapshot.deref(), &mut proc_entry)?;
            loop {
                if proc_entry.th32ProcessID == pid {
                    return Ok(proc_entry);
                }
                Process32Next(*snapshot.deref(), &mut proc_entry)?;
            }
        }
    }

    fn get_parent_pid(pid: u32) -> core::Result<u32> {
        let process = get_process_entry(pid)?;
        Ok(process.th32ParentProcessID)
    }

    /// returns [`bool`]:
    /// - [`true`] if the executable was launched with a double click
    /// - [`false`] if the executable was launched from the terminal
    pub fn started_by_explorer() -> bool {
        match get_process_entry(get_parent_pid(id()).unwrap()) {
            Ok(proc_entry) => {
                let maybe_exe = unsafe {
                    let u8_str = transmute::<&[i8; 260], &[u8; 260]>(&proc_entry.szExeFile);
                    CStr::from_bytes_until_nul(u8_str)
                };
                if let Ok(exe) = maybe_exe {
                    exe.to_str().unwrap_or("") == "explorer.exe"
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }
}

#[cfg(not(target_os = "windows"))]
mod launch_check {
    /// returns [`bool`]:
    /// on systems different from Windows this function always returns [`false`]
    /// open an issue with a proposal if you are interested in the feature
    pub fn started_by_explorer() -> bool {
        false
    }
}

pub use launch_check::*;

#[cfg(test)]
mod test {
    use super::started_by_explorer;
    #[test]
    fn test_started_by_explorer() {
        assert!(!started_by_explorer());
    }
}