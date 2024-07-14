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

    pub fn started_by_explorer() -> bool {
        match get_process_entry(get_parent_pid(id()).unwrap()) {
            Ok(proc_entry) => {
                if let Ok(exe) = unsafe {
                    CStr::from_bytes_until_nul(transmute::<&[i8; 260], &[u8; 260]>(
                        &proc_entry.szExeFile,
                    ))
                } {
                    println!("{exe:?}");
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
    pub fn started_by_explorer() -> bool {
        false
    }
}

pub use launch_check::*;
