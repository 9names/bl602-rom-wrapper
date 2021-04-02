// mod rom_lookup;
use crate::rom_lookup::{rom_lookup, RomIndex};
use crate::rom::SF_Ctrl_Owner_Type;

#[inline(always)]
pub fn SF_Ctrl_Set_Owner(owner: SF_Ctrl_Owner_Type) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(SF_Ctrl_Owner_Type)>(rom_lookup(
            RomIndex::SF_Ctrl_Set_Owner,
        ))
    };
    romdriver_func(owner)
}

#[inline(always)]
pub fn SF_Ctrl_Set_Flash_Image_Offset(offset: u32) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(u32)>(rom_lookup(
            RomIndex::SF_Ctrl_Set_Flash_Image_Offset,
        ))
    };
    romdriver_func(offset)
}
