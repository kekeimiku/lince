use std::{fs::OpenOptions, io::BufWriter};

use consts::MAX_BUF_SIZE;
use vmmap::{Process, ProcessInfo};

use super::{a::create_pointer_map_helper, cmd::SubCommandDisk};

impl SubCommandDisk {
    pub fn init(self) -> Result<(), Box<dyn std::error::Error>> {
        let SubCommandDisk { pid, out } = self;
        let proc = Process::open(pid)?;
        let name = proc
            .app_path()
            .file_name()
            .and_then(|f| f.to_str())
            .ok_or("get app_name error")?;

        let out = match out {
            Some(file) => OpenOptions::new().write(true).append(true).create(true).open(file),
            None => OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(format!("{name}-{pid}.dump")),
        }?;
        let out = BufWriter::with_capacity(MAX_BUF_SIZE, out);

        Ok(create_pointer_map_helper(proc, out)?)
    }
}