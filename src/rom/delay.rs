use crate::rom_lookup::{rom_lookup, RomIndex};

#[inline(always)]
pub fn ASM_Delay_Us(
    core: u32,
    cnt: u32,
) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(u32, u32)>(
            rom_lookup(RomIndex::ASM_Delay_Us),
        )
    };
    romdriver_func(core, cnt)
}

#[inline(always)]
pub fn BL602_Delay_US(
    cnt: u32
) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(u32)>(
            rom_lookup(RomIndex::BL602_Delay_US),
        )
    };
    romdriver_func(cnt)
}

#[inline(always)]
pub fn BL602_Delay_MS(
    cnt: u32
) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(u32)>(
            rom_lookup(RomIndex::BL602_Delay_MS),
        )
    };
    romdriver_func(cnt)
}