use std::fs::File;
use std::os::unix::fs::FileExt;

/// Write `content` starting from the given `offset`
///
/// # Errors
///
/// See [`FileExt::write_at()`]
pub fn write_at(file: &mut File, content: &[u8], offset: u64) -> std::io::Result<usize> {
	file.write_at(content, offset)
}
