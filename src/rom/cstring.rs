#![allow(clippy::upper_case_acronyms)]
use crate::rom_lookup::{rom_lookup, RomIndex};
use cty::{c_int, c_void};
#[allow(dead_code)]
struct Cstring {
    BL602_MemCpy_ptr: extern "C" fn(*mut c_void, *const c_void, u32) -> *mut c_void,
    BL602_MemCpy4_ptr: extern "C" fn(*mut u32, *const u32, u32) -> *mut u32,
    BL602_MemCpy_Fast_ptr: extern "C" fn(*mut c_void, *const c_void, u32) -> *mut c_void,
    BL602_MemSet_ptr: extern "C" fn(*mut c_void, u8, u32) -> *mut c_void,
    BL602_MemSet4_ptr: extern "C" fn(*mut u32, u32, u32) -> *mut u32,
    BL602_MemCmp_ptr: extern "C" fn(*const c_void, *const c_void, u32) -> c_int,
}

impl Cstring {
    #[allow(dead_code)]
    pub fn new() -> Cstring {
        Cstring {
            BL602_MemCpy_ptr: unsafe {
                core::mem::transmute::<
                    *const (),
                    extern "C" fn(*mut c_void, *const c_void, u32) -> *mut c_void,
                >(rom_lookup(RomIndex::BL602_MemCpy))
            },
            BL602_MemCpy4_ptr: unsafe {
                core::mem::transmute::<
                    *const (),
                    extern "C" fn(*mut u32, *const u32, u32) -> *mut u32,
                >(rom_lookup(RomIndex::BL602_MemCpy4))
            },
            BL602_MemCpy_Fast_ptr: unsafe {
                core::mem::transmute::<
                    *const (),
                    extern "C" fn(*mut c_void, *const c_void, u32) -> *mut c_void,
                >(rom_lookup(RomIndex::BL602_MemCpy_Fast))
            },
            BL602_MemSet_ptr: unsafe {
                core::mem::transmute::<*const (), extern "C" fn(*mut c_void, u8, u32) -> *mut c_void>(
                    rom_lookup(RomIndex::BL602_MemSet),
                )
            },
            BL602_MemSet4_ptr: unsafe {
                core::mem::transmute::<*const (), extern "C" fn(*mut u32, u32, u32) -> *mut u32>(
                    rom_lookup(RomIndex::BL602_MemSet4),
                )
            },
            BL602_MemCmp_ptr: unsafe {
                core::mem::transmute::<
                    *const (),
                    extern "C" fn(*const c_void, *const c_void, u32) -> c_int,
                >(rom_lookup(RomIndex::BL602_MemCmp))
            },
        }
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn BL602_MemCpy(&self, dst: *mut c_void, src: *const c_void, n: u32) -> *mut c_void {
        (self.BL602_MemCpy_ptr)(dst, src, n)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn BL602_MemCpy4(&self, dst: *mut u32, src: *const u32, n: u32) -> *mut u32 {
        (self.BL602_MemCpy4_ptr)(dst, src, n)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn BL602_MemCpy_Fast(&self, pdst: *mut c_void, psrc: *const c_void, n: u32) -> *mut c_void {
        (self.BL602_MemCpy_Fast_ptr)(pdst, psrc, n)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn BL602_MemSet(&self, s: *mut c_void, c: u8, n: u32) -> *mut c_void {
        (self.BL602_MemSet_ptr)(s, c, n)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn BL602_MemSet4(&self, dst: *mut u32, val: u32, n: u32) -> *mut u32 {
        (self.BL602_MemSet4_ptr)(dst, val, n)
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn BL602_MemCmp(&self, s1: *const c_void, s2: *const c_void, n: u32) -> c_int {
        (self.BL602_MemCmp_ptr)(s1, s2, n)
    }
}

#[allow(dead_code)]
#[inline(always)]
pub fn BL602_MemCpy(dst: *mut c_void, src: *const c_void, n: u32) -> *mut c_void {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut c_void, *const c_void, u32) -> *mut c_void,
        >(rom_lookup(RomIndex::BL602_MemCpy))
    };
    romdriver_func(dst, src, n)
}

#[allow(dead_code)]
#[inline(always)]
pub fn BL602_MemCpy4(dst: *mut u32, src: *const u32, n: u32) -> *mut u32 {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut u32, *const u32, u32) -> *mut u32>(
            rom_lookup(RomIndex::BL602_MemCpy4),
        )
    };
    romdriver_func(dst, src, n)
}

#[allow(dead_code)]
#[inline(always)]
pub fn BL602_MemCpy_Fast(pdst: *mut c_void, psrc: *const c_void, n: u32) -> *mut c_void {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut c_void, *const c_void, u32) -> *mut c_void,
        >(rom_lookup(RomIndex::BL602_MemCpy_Fast))
    };
    romdriver_func(pdst, psrc, n)
}

#[allow(dead_code)]
#[inline(always)]
pub fn BL602_MemSet(s: *mut c_void, c: u8, n: u32) -> *mut c_void {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut c_void, u8, u32) -> *mut c_void>(
            rom_lookup(RomIndex::BL602_MemSet),
        )
    };
    romdriver_func(s, c, n)
}

#[allow(dead_code)]
#[inline(always)]
pub fn BL602_MemSet4(dst: *mut u32, val: u32, n: u32) -> *mut u32 {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut u32, u32, u32) -> *mut u32>(
            rom_lookup(RomIndex::BL602_MemSet4),
        )
    };
    romdriver_func(dst, val, n)
}

#[allow(dead_code)]
#[inline(always)]
pub fn BL602_MemCmp(s1: *const c_void, s2: *const c_void, n: u32) -> c_int {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*const c_void, *const c_void, u32) -> c_int>(
            rom_lookup(RomIndex::BL602_MemCmp),
        )
    };
    romdriver_func(s1, s2, n)
}
