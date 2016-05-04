use std::ffi::{CString, CStr};
use std::ptr;

use error::BfdError;
use ffi;
use arch::ArchInfo;
use section::Section;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Format {
    Unknown,
    Object,
    Archive,
    Core,
}

impl Format {
    pub fn from_raw(fmt: ffi::bfd_format) -> Format {
        use ffi::Enum_bfd_format::*;
        match fmt {
            bfd_object => Format::Object,
            bfd_archive => Format::Archive,
            bfd_core => Format::Core,
            _ => Format::Unknown
        }
    }

    pub fn to_raw(&self) -> ffi::Enum_bfd_format {
        use ffi::Enum_bfd_format::*;
        match *self {
            Format::Object => bfd_object,
            _ => bfd_unknown,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Endian {
    Unknown,
    Big,
    Little,
}

pub struct Bfd {
    bfd_type: Format,
    raw: *mut ffi::bfd,
}

macro_rules! bfd_getter (
    ($n:ident, $t:ty) => {
        pub fn $n(&self) -> $t {
            unsafe { (*self.raw).$n }
        }
    };
);

impl Bfd {
    pub fn open(filename: &str, bfd_type: Format) -> Result<Bfd, BfdError> {
        ::init();
        let bfd = unsafe {
            let name = CString::new(filename).unwrap();
            ffi::bfd_openr(name.as_ptr(), ptr::null())
        };
        if bfd.is_null() {
            Err(BfdError::last_bfd_error())
        } else {
            if unsafe {ffi::bfd_check_format(bfd, bfd_type.to_raw()) } != 0 {
                Ok(Bfd {
                    bfd_type: bfd_type,
                    raw: bfd,
                })
            } else {
                Err(BfdError::last_bfd_error())
            }
        }
    }

    pub fn format(&self) -> String {
        unsafe { ::cstr_to_string((*(*self.raw).xvec).name) }
    }

    pub fn endian(&self) -> Endian {
        use ffi::Enum_bfd_endian::*;
        match unsafe {(*(*self.raw).xvec).byteorder} {
            BFD_ENDIAN_BIG => Endian::Big,
            BFD_ENDIAN_LITTLE => Endian::Little,
            _ => Endian::Unknown
        }
    }

    pub fn arch_info(&self) -> ArchInfo {
        ArchInfo::from_raw_bfd(self.raw)
    }

    pub fn sections(&self) -> Vec<Section> {
        let mut sections = Vec::<Section>::new();
        let mut t = Section::from_raw(unsafe { (*self.raw).sections }, &self);
        while let Some(s) = t {
            t = s.next();
            sections.push(s);
        }
        sections
    }

    bfd_getter!(section_count, u32);
    bfd_getter!(start_address, ::Vma);
    bfd_getter!(symcount, u32);
    bfd_getter!(dynsymcount, u32);
}

impl Drop for Bfd {
    fn drop(&mut self) {
        unsafe { ffi::bfd_close(self.raw); }
    }
}


#[cfg(test)]
mod tests {
    use {Bfd, Endian, Format};
    #[test]
    fn open_bad() {
        assert!(Bfd::open("BAD_FILE", Format::Object).is_err());
    }

    #[test]
    fn open_object() {
        let bfd = Bfd::open("data/test_x86", Format::Object).unwrap();
        assert_eq!(bfd.format(), "elf32-i386");
        assert_eq!(bfd.arch_info().printable_arch_mach(), "i386");

        let bfd = Bfd::open("data/test_x86_64", Format::Object).unwrap();
        assert_eq!(bfd.format(), "elf64-x86-64");
        assert_eq!(bfd.arch_info().printable_arch_mach(), "i386:x86-64");
        assert_eq!(bfd.section_count(), 26);

        let bfd = Bfd::open("data/test_aarch64", Format::Object).unwrap();
        assert_eq!(bfd.format(), "elf64-littleaarch64");
        assert_eq!(bfd.arch_info().printable_arch_mach(), "aarch64");

        let bfd = Bfd::open("data/test_pe64", Format::Object).unwrap();
        assert_eq!(bfd.format(), "pei-x86-64");
        assert_eq!(bfd.arch_info().printable_arch_mach(), "i386:x86-64");
        println!("{:?}", bfd.sections());
    }
}

