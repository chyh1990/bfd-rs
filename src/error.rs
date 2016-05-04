use std::{io, fmt, error};
use std::ffi::CStr;
use ffi;

impl fmt::Debug for ffi::bfd_error_type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = unsafe { CStr::from_ptr(ffi::bfd_errmsg(*self)) };
        write!(f, "{}", msg.to_string_lossy())
    }
}

#[derive(Debug)]
pub enum BfdError {
    Io(io::Error),
    Bfd(ffi::bfd_error_type),
}

impl fmt::Display for BfdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BfdError::Io(ref err) => err.fmt(f),
            BfdError::Bfd(err) => {
                let msg = unsafe { CStr::from_ptr(ffi::bfd_errmsg(err)) };
                write!(f, "Bfd error: {}", msg.to_string_lossy())
            },
        }
    }
}

impl error::Error for BfdError {
    fn description(&self) -> &str {
        use ffi::Enum_bfd_error::*;
        match *self {
            BfdError::Io(ref err) => err.description(),
            BfdError::Bfd(err) => {
                match err {
                    bfd_error_no_error => "No error",
                    bfd_error_system_call => "System call",
                    bfd_error_invalid_target => "Invalid target",
                    bfd_error_wrong_format => "Wrong format",
                    bfd_error_wrong_object_format => "Wrong object format",
                    bfd_error_invalid_operation => "Invalid operation",
                    bfd_error_no_memory => "No memory",
                    bfd_error_no_symbols => "No symbols",
                    bfd_error_no_armap => "No armap",
                    bfd_error_no_more_archived_files => "No more archived files",
                    bfd_error_malformed_archive => "Malformed archive",
                    bfd_error_missing_dso => "Missing dso",
                    bfd_error_file_not_recognized => "File not recognized",
                    bfd_error_file_ambiguously_recognized => "File ambiguously recognized",
                    bfd_error_no_contents => "No contents",
                    bfd_error_nonrepresentable_section => "Nonrepresentable section",
                    bfd_error_no_debug_section => "No debug section",
                    bfd_error_bad_value => "Bad value",
                    bfd_error_file_truncated => "File truncated",
                    bfd_error_file_too_big => "File too big",
                    bfd_error_on_input => "On input",
                    _ => "#<Invalid error code>"
                }
            },
        }
    }
}

impl BfdError {
    pub fn last_bfd_error() -> BfdError {
        BfdError::Bfd(unsafe {ffi::bfd_get_error()})
    }
}

