// mod rom_lookup;
use crate::rom_lookup::{rom_lookup, RomIndex};
use crate::sflash::SF_Ctrl_Mode_Type;

use crate::sflash::BL_Err_Type;
use crate::sflash::SF_Ctrl_Owner_Type;

// extern "C" {
//     pub fn SF_Cfg_Flash_Identify(
//         callFromFlash: u8,
//         autoScan: u32,
//         flashPinCfg: u32,
//         restoreDefault: u8,
//         pFlashCfg: *mut SPI_Flash_Cfg_Type,
//     ) -> u32;
// }

pub fn SF_Cfg_Flash_Identify(
    callFromFlash: u8,
    autoScan: u32,
    flashPinCfg: u32,
    restoreDefault: u8,
    pFlashCfg: *mut SF_Ctrl_Mode_Type,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(u8, u32, u32, u8, *mut SF_Ctrl_Mode_Type) -> u32,
        >(rom_lookup(RomIndex::SF_Cfg_Flash_Identify))
    };
    romdriver_func(
        callFromFlash,
        autoScan,
        flashPinCfg,
        restoreDefault,
        pFlashCfg,
    )
}

pub fn SF_Cfg_Init_Flash_Gpio(
    flashPinCfg: u8, restoreDefault: u8
) {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(u8, u8),
        >(rom_lookup(RomIndex::SF_Cfg_Init_Flash_Gpio))
    };
    romdriver_func(
        flashPinCfg,
        restoreDefault,
    )
}

pub fn SF_Ctrl_Set_Owner(
    owner: SF_Ctrl_Owner_Type,
) {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(SF_Ctrl_Owner_Type),
        >(rom_lookup(RomIndex::SF_Ctrl_Set_Owner))
    };
    romdriver_func(
        owner
    )
}