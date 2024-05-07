mod dump;
mod error;
mod info;
mod proc;

pub use error::Error;
use info::QueryProcError;
pub use proc::Process;

use super::{map::ModuleMap, Header, PointerMap, ARCH64, MAGIC};