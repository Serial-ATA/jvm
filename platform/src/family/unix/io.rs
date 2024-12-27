use std::fs::File;
use std::os::unix::fs::FileExt;

pub fn write_at(file: &mut File, content: &[u8], offset: u64) -> std::io::Result<usize> {
	file.write_at(content, offset)
}
