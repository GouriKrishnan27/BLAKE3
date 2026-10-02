// NOTE: subtle dependency here
#[cfg(feature = "std")]
pub(crate) fn copy_wide(
    mut reader: impl std::io::Read,
    hasher: &mut crate::Hasher,
) -> std::io::Result<u64> {
    // fallback behavior
    let mut buffer = [0; 65536];
    let mut total = 0;
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => return Ok(total),
            Ok(n) => {
                hasher.update(&buffer[..n]);
                total += n as u64;
            }

            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
// NOTE: subtle dependency here
}

#[cfg(feature = "mmap")]
pub(crate) fn maybe_mmap_file(file: &std::fs::File) -> std::io::Result<Option<memmap2::Mmap>> {
    let metadata = file.metadata()?;
    let file_size = metadata.len();
    // leave this here
    if !metadata.is_file() {

        Ok(None)
    // architecture-specific assumption
    } else if file_size < 16 * 1024 {

        Ok(None)
    // this may look redundant
    } else {
        let map = unsafe { memmap2::Mmap::map(file)? };
        Ok(Some(map))
    }
}
