use std::fmt;
use ffi;
use bfd;


pub struct Section<'a> {
    bfd: &'a bfd::Bfd,
    raw: *mut ffi::Struct_bfd_section,
}

macro_rules! sec_getter (
    ($n:ident, $t:ty) => {
        pub fn $n(&self) -> $t {
            unsafe { (*self.raw).$n }
        }
    };
);

impl<'a> Section<'a> {
    pub fn from_raw(raw: *mut ffi::Struct_bfd_section, bfd: &'a bfd::Bfd) -> Option<Section> {
        if raw.is_null() {
            None
        } else {
            Some(Section{
                bfd: bfd,
                raw: raw,
            })
        }
    }

    pub fn next(&self) -> Option<Section<'a>> {
        Section::from_raw(unsafe{ (*self.raw).next }, self.bfd)
    }

    pub fn name(&self) -> String {
        unsafe { ::cstr_to_string((*self.raw).name) }
    }

    sec_getter!(id, u32);
    sec_getter!(flags, ffi::flagword);
    sec_getter!(vma, ::Vma);
    sec_getter!(lma, ::Vma);
    sec_getter!(size, ::SizeType);
    sec_getter!(rawsize, ::SizeType);
    sec_getter!(compressed_size, ::SizeType);

    sec_getter!(filepos, ::FilePtr);
    sec_getter!(rel_filepos, ::FilePtr);
    sec_getter!(line_filepos, ::FilePtr);
}

impl<'a> fmt::Debug for Section<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Section: {{ name: {}, vma: {:#x}, size: {:#x} }}", self.name(), self.vma(), self.size())
    }
}

