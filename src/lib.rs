use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct InMemFilePath {
    _file: File,
    path: PathBuf,
}

impl AsRef<Path> for InMemFilePath {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

#[cfg(target_os = "linux")]
impl InMemFilePath {
    pub fn new(contents: &[u8]) -> Result<Self, &'static str> {
        let mfd = memfd::MemfdOptions::default()
            .create("fuzz-file")
            .map_err(|_| "memfd error")?;

        let fd = mfd.as_raw_fd();
        let path = PathBuf::from(format!("/proc/self/fd/{fd}"));

        let mut file = mfd.into_file();
        if file.write_all(contents).is_err() {
            println!("could not write to memfd file!");
            return Err("file write error");
        }

        if file.seek(SeekFrom::Start(0)).is_err() {
            println!("failed to seek!");
            return Err("could not seek to start");
        }

        Ok(Self { _file: file, path })
    }
}
