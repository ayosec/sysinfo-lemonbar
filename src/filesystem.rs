use libc;
use std::io;
use std::mem;
use std::ffi::CString;

#[derive(Debug)]
pub struct FileSysteUsage<'a> {
    path: &'a str,
    block_size: u64,
    block_free: u64,
}

pub fn get_info<'a, S: AsRef<str>>(paths: &'a[S]) -> Vec<FileSysteUsage<'a>> {
    paths
        .iter()
        .map(|p| read_fs(p.as_ref()))
        .filter_map(|fs| fs.ok())
        .collect()
}

fn read_fs<'a>(path: &'a str) -> io::Result<FileSysteUsage> {
    let mut stat;
    unsafe {
        stat = mem::zeroed();
        let c_path = CString::new(path)?;
        if libc::statfs(c_path.as_ptr(), &mut stat) == -1 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(
        FileSysteUsage {
            path: path,
            block_size: stat.f_blocks,
            block_free: stat.f_bavail,
        }
    )
}
