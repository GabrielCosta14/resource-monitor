use anyhow::Result;
use libc::{c_int, rusage_info_v2, RUSAGE_INFO_V2};

#[derive(Default, Clone, Copy)]
pub struct Io {
    pub read_bytes:  u64,
    pub write_bytes: u64,
}

pub fn stats() -> Result<Io> {
    unsafe {
        let mut info = std::mem::MaybeUninit::<rusage_info_v2>::uninit();
        let ret = libc::proc_pid_rusage(
            std::process::id() as c_int,
            RUSAGE_INFO_V2,
            info.as_mut_ptr() as *mut _,
        );
        if ret != 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        let i = info.assume_init();
        Ok(Io {
            read_bytes:  i.ri_diskio_bytesread,
            write_bytes: i.ri_diskio_byteswritten,
        })
    }
}
