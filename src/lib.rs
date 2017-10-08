#[cfg(windows)]
extern crate kernel32;
#[cfg(not(windows))]
extern crate libc;
extern crate time;

use time::Duration;
#[cfg(not(windows))]
use std::mem;

#[cfg(target_os = "linux")]
pub fn get() -> Result<Duration, String> {
    let mut info: libc::sysinfo = unsafe { mem::zeroed() };
    let ret = unsafe { libc::sysinfo(&mut info) };
    if ret == 0 {
        Ok(Duration::seconds(info.uptime as i64))
    } else {
        Err("sysinfo failed".to_string())
    }
}

#[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
pub fn get() -> Result<Duration, String> {
    let mut request = [libc::CTL_KERN, libc::KERN_BOOTTIME];
    let mut boottime: libc::timeval = unsafe { mem::zeroed() };
    let mut size: libc::size_t = mem::size_of_val(&boottime) as libc::size_t;
    let ret = unsafe {
        libc::sysctl(
            &mut request[0],
            2,
            &mut boottime as *mut libc::timeval as *mut libc::c_void,
            &mut size,
            std::ptr::null_mut(),
            0,
        )
    };
    if ret == 0 {
        Ok((time::now().to_timespec() - time::Timespec::new(boottime.tv_sec, boottime.tv_usec * 1000)))
    } else {
        Err("sysctl failed".to_string())
    }
}

#[cfg(target_os = "windows")]
pub fn get() -> Result<Duration, String> {
    let ret: u64 = unsafe { kernel32::GetTickCount64() };
    Ok(Duration::milliseconds(ret as i64))
}
