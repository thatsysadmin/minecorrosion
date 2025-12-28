pub mod configure;
pub mod database;
pub mod launch;

/// breakpoint_trap_option
pub fn breakpoint_trap_option<T>(call: Option<T>) -> Option<T> {
    match call {
        Some(x) => Some(x),
        None => {
            None
        }
    }
}

/// breakpoint_trap_result
pub fn breakpoint_trap_result<T, E>(call: Result<T, E>) -> Result<T, E> {
    match call {
        Ok(x) => Ok(x),
        Err(e) => {
            Err(e)
        }
    }
}