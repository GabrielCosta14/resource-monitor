use std::fs;

pub fn count() -> std::io::Result<usize> {
    // subtract 1 to ignore the fd created by read_dir itself
    fs::read_dir("/dev/fd").map(|iter| iter.count().saturating_sub(1))
}
