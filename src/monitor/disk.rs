use anyhow::Result;
use libc::statfs;
use std::ffi::CString;

pub fn used_vs_total_gib() -> Result<(u64, u64)> {
    let path = CString::new("/")?;
    let mut s: statfs = unsafe { std::mem::zeroed() };
    let ret = unsafe { libc::statfs(path.as_ptr(), &mut s) };
    if ret != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let total = s.f_blocks as u64 * s.f_bsize as u64;
    let avail = s.f_bavail as u64 * s.f_bsize as u64;
    let used = total.saturating_sub(avail);
    Ok((used / 1_073_741_824, total / 1_073_741_824)) // to GiB
}
