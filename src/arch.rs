use std::ffi::CStr;
use ffi;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Arch {
    Unknown,
}

pub struct ArchInfo {
    parent: *mut ffi::Struct_bfd,
    raw: *const ffi::Struct_bfd_arch_info
}

macro_rules! arch_getter (
    ($n:ident, $t:ty) => {
        pub fn $n(&self) -> $t {
            unsafe { (*self.raw).$n }
        }
    };
);

impl ArchInfo {
    pub fn from_raw_bfd(raw: *mut ffi::bfd) -> ArchInfo {
        ArchInfo {
            parent: raw,
            raw: unsafe {ffi::bfd_get_arch_info(raw)}
        }
    }

    pub fn printable_arch_mach(&self) -> String {
        let cstr = unsafe {
            let arch = ffi::bfd_get_arch(self.parent);
            let mach = ffi::bfd_get_mach(self.parent);
            CStr::from_ptr(ffi::bfd_printable_arch_mach(arch, mach))
        };
        cstr.to_string_lossy().into_owned()
    }

    pub fn arch_name(&self) -> String {
        let cstr = unsafe { CStr::from_ptr((*self.raw).arch_name) };
        cstr.to_string_lossy().into_owned()
    }

    arch_getter!(bits_per_word, i32);
    arch_getter!(bits_per_address, i32);
    arch_getter!(bits_per_byte, i32);
    arch_getter!(mach, ::std::os::raw::c_ulong);
    arch_getter!(section_align_power, u32);
}

