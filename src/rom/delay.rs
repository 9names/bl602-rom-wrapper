use crate::rom_lookup::{rom_lookup, RomIndex};

#[allow(dead_code)]
struct Delay {
    ASM_Delay_Us_ptr: extern "C" fn(u32, u32),
    BL602_Delay_US_ptr: extern "C" fn(u32),
    BL602_Delay_MS_ptr: extern "C" fn(u32),
}

impl Delay {
    #[allow(dead_code)]
    pub fn new() -> Delay {
        Delay {
            ASM_Delay_Us_ptr: unsafe {
                core::mem::transmute::<*const (), extern "C" fn(u32, u32)>(rom_lookup(
                    RomIndex::ASM_Delay_Us,
                ))
            },
            BL602_Delay_US_ptr: unsafe {
                core::mem::transmute::<*const (), extern "C" fn(u32)>(rom_lookup(
                    RomIndex::BL602_Delay_US,
                ))
            },
            BL602_Delay_MS_ptr: unsafe {
                core::mem::transmute::<*const (), extern "C" fn(u32)>(rom_lookup(
                    RomIndex::BL602_Delay_MS,
                ))
            },
        }
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn ASM_Delay_Us(&self, core: u32, cnt: u32) {
        (self.ASM_Delay_Us_ptr)(core, cnt)
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn BL602_Delay_US(&self, cnt: u32) {
        (self.BL602_Delay_US_ptr)(cnt)
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn BL602_Delay_MS(&self, cnt: u32) {
        (self.BL602_Delay_MS_ptr)(cnt)
    }
}

#[inline(always)]
pub fn ASM_Delay_Us(core: u32, cnt: u32) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(u32, u32)>(rom_lookup(
            RomIndex::ASM_Delay_Us,
        ))
    };
    romdriver_func(core, cnt)
}

#[inline(always)]
pub fn BL602_Delay_US(cnt: u32) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(u32)>(rom_lookup(RomIndex::BL602_Delay_US))
    };
    romdriver_func(cnt)
}

#[inline(always)]
pub fn BL602_Delay_MS(cnt: u32) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(u32)>(rom_lookup(RomIndex::BL602_Delay_MS))
    };
    romdriver_func(cnt)
}
