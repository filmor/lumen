#[cfg(test)]
mod test;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

use lumen_runtime_macros::native_implemented_function;

use lumen_runtime::time::{monotonic, system, Unit::Native};

#[native_implemented_function(time_offset/0)]
pub fn native(process: &Process) -> exception::Result<Term> {
    let system_time = system::time(Native);
    let monotonic_time = monotonic::time(Native);

    Ok(process.integer(system_time - monotonic_time)?)
}