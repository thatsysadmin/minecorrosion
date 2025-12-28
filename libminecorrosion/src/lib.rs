pub mod configure;
pub mod database;
pub mod launch;

/// breakpoint_trap_option
pub fn breakpoint_trap_option<T>(call: Option<T>) -> Option<T> {
    #[cfg(debug_assertions)]
    match call {
        Some(x) => Some(x),
        None => {
            None
        }
    }

    #[cfg(not(debug_assertions))]
    call
}

/// breakpoint_trap_result
pub fn breakpoint_trap_result<T, E>(call: Result<T, E>) -> Result<T, E> {
    #[cfg(debug_assertions)]
    match call {
        Ok(x) => Ok(x),
        Err(e) => {
            Err(e)
        }
    }

    #[cfg(not(debug_assertions))]
    call
}