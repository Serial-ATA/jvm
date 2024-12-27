use std::fs::File;
use std::os::windows::fs::FileExt;

pub fn write_at(file: &mut File, content: &[u8], offset: u64) -> std::io::Result<usize> {
	file.seek_write(content, offset)
}
