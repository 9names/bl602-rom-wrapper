#![allow(clippy::upper_case_acronyms)]
use crate::rom::SF_Ctrl_Owner_Type;
use crate::rom_lookup::{rom_lookup, RomIndex};

#[inline(always)]
pub fn SF_Ctrl_Set_Owner(owner: SF_Ctrl_Owner_Type) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(SF_Ctrl_Owner_Type)>(rom_lookup(
            RomIndex::SF_Ctrl_Set_Owner,
        ))
    };
    romdriver_func(owner)
}

// NOTE: the SF_Ctrl_Set_Flash_Image_Offset and SF_Ctrl_Get_Flash_Image_Offset functions would
// be good candidates for replacing with Rust calls the bl602-hal as they're just register lookups
// The corresponding register is SF_CTRL_BASE.SF_CTRL_SF_ID0_OFFSET, and it takes a u32

/// Set the starting location in SPI Flash for mapping into CPU address space
///
/// `offset` - the offset from the start of SPI flash to where the first memory
/// mapped region starts.
///
/// *Example*: SF_Ctrl_Set_Flash_Image_Offset(0) will make SPI address 0x0
/// correspond to 0x2300_0000 in CPU address space
#[inline(always)]
pub fn SF_Ctrl_Set_Flash_Image_Offset(offset: u32) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(u32)>(rom_lookup(
            RomIndex::SF_Ctrl_Set_Flash_Image_Offset,
        ))
    };
    romdriver_func(offset)
}

// uint32_t ATTR_TCM_SECTION SF_Ctrl_Get_Flash_Image_Offset(void)
// {
//     return BL_RD_REG(SF_CTRL_BASE,SF_CTRL_SF_ID0_OFFSET);
// }
#[inline(always)]
pub fn SF_Ctrl_Get_Flash_Image_Offset() -> u32 {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn() -> u32>(rom_lookup(
            RomIndex::SF_Ctrl_Get_Flash_Image_Offset,
        ))
    };
    romdriver_func()
}
