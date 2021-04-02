use crate::rom_lookup::{rom_lookup, RomIndex};
use crate::rom::{BL_Err_Type, SF_Ctrl_Mode_Type, SPI_Flash_Cfg_Type};

pub fn SF_Cfg_Get_Flash_Cfg_Need_Lock(
    flashID: u32,
    pFlashCfg: *mut SPI_Flash_Cfg_Type,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(u32, *mut SPI_Flash_Cfg_Type) -> u32>(
            rom_lookup(RomIndex::SF_Cfg_Get_Flash_Cfg_Need_Lock),
        )
    };
    romdriver_func(flashID, pFlashCfg)
}

pub fn SF_Cfg_Restore_GPIO17_Fun(fun: u8) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(u8)>(rom_lookup(
            RomIndex::SF_Cfg_Restore_GPIO17_Fun,
        ))
    };
    romdriver_func(fun)
}

#[inline(always)]
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

pub fn SF_Cfg_Init_Ext_Flash_Gpio(extFlashPin: u8) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(u8)>(rom_lookup(
            RomIndex::SF_Cfg_Init_Ext_Flash_Gpio,
        ))
    };
    romdriver_func(extFlashPin)
}

#[inline(always)]
pub fn SF_Cfg_Init_Flash_Gpio(flashPinCfg: u8, restoreDefault: u8) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(u8, u8)>(rom_lookup(
            RomIndex::SF_Cfg_Init_Flash_Gpio,
        ))
    };
    romdriver_func(flashPinCfg, restoreDefault)
}

pub fn SF_Cfg_Init_Internal_Flash_Gpio() {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn()>(rom_lookup(
            RomIndex::SF_Cfg_Init_Internal_Flash_Gpio,
        ))
    };
    romdriver_func()
}
pub fn SF_Cfg_Deinit_Ext_Flash_Gpio(extFlashPin: u8) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(u8)>(rom_lookup(
            RomIndex::SF_Cfg_Deinit_Ext_Flash_Gpio,
        ))
    };
    romdriver_func(extFlashPin)
}
