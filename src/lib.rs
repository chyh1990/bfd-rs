extern crate libc;
#[macro_use]
extern crate bitflags;

use std::sync::{Once, ONCE_INIT};
use std::ffi::CStr;

#[allow(dead_code, non_camel_case_types)]
pub mod ffi;

mod error;
mod bfd;
mod arch;
mod section;

fn init() {
    static INIT: Once = ONCE_INIT;

    INIT.call_once(|| {
        unsafe {
            ffi::bfd_init();
        }
    });
}

unsafe fn cstr_to_string(s: *const ::std::os::raw::c_char) -> String {
    let cstr = CStr::from_ptr(s);
    cstr.to_string_lossy().into_owned()
}

pub type Vma = ffi::bfd_vma;
pub type SignedVma = ffi::bfd_signed_vma;
pub type SizeType = ffi::bfd_size_type;
pub type FilePtr = ffi::file_ptr;

pub const SEC_NO_FLAGS: ffi::flagword = 0x000;
pub const SEC_ALLOC: ffi::flagword = 0x001;
pub const SEC_LOAD: ffi::flagword = 0x002;
pub const SEC_RELOC: ffi::flagword = 0x004;
pub const SEC_READONLY: ffi::flagword = 0x008;
pub const SEC_CODE: ffi::flagword = 0x010;
pub const SEC_DATA: ffi::flagword = 0x020;
pub const SEC_ROM: ffi::flagword = 0x040;

pub use error::BfdError;
pub use bfd::Bfd;
pub use bfd::Format;
pub use bfd::Endian;
pub use arch::ArchInfo;
pub use section::Section;
