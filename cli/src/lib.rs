pub mod cmd;
pub mod consts;
pub mod create_map;
pub mod pointer_map;
pub mod pointer_path;
pub mod scanner;
pub mod scanner_map;
pub mod spinner;

use std::{array::TryFromSliceError, ffi::OsStr, fs::File, io::Read, path::PathBuf};

use consts::EXE;
use vmmap::VirtualQuery;

#[derive(Default, Clone, Debug, bincode::Encode, bincode::Decode)]
pub struct Map {
    pub start: usize,
    pub end: usize,
    pub size: usize,
    pub is_read: bool,
    pub is_write: bool,
    pub is_exec: bool,
    pub is_stack: bool,
    pub is_heap: bool,
    pub path: Option<PathBuf>,
    pub name: String,
}

impl core::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.path
                .as_ref()
                .and_then(|f| f.file_name())
                .unwrap_or_else(|| OsStr::new("[misc]"))
                .to_string_lossy()
        )
    }
}

impl<T> From<T> for Map
where
    T: VirtualQuery,
{
    fn from(value: T) -> Self {
        Self {
            start: value.start(),
            end: value.end(),
            size: value.size(),
            is_read: value.is_read(),
            is_write: value.is_write(),
            is_exec: value.is_exec(),
            is_stack: value.is_stack(),
            is_heap: value.is_heap(),
            path: value.path().map(PathBuf::from),
            name: value.name().to_string(),
        }
    }
}

impl Map {
    pub fn is_exe(&self) -> bool {
        if !self.is_read {
            return false;
        }

        let Some(path) = &self.path else {
        return false;
    };

        #[cfg(target_os = "linux")]
        if path.starts_with("/dev") || path.starts_with("/usr")
        // || matches!(self.name.as_str(), "[vvar]" | "[vdso]" | "[vsyscall]")
        {
            // println!("{} {}", self.start, self.name);
            return false;
        }

        #[cfg(target_os = "macos")]
        if path.starts_with("/usr") {
            return false;
        }

        if let Ok(mut file) = File::open(path) {
            let mut buf = [0; 4];
            if file.read_exact(&mut buf).is_ok() {
                return EXE.eq(&buf);
            }
        }
        false
    }
}

pub fn bytes_to_usize(buf: &[u8]) -> Result<usize, String> {
    Ok(usize::from_le_bytes(buf.try_into().map_err(|e: TryFromSliceError| e.to_string())?))
}

pub const fn wrap_add(u: usize, i: i16) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as usize)
    } else {
        u.checked_add(i as usize)
    }
}