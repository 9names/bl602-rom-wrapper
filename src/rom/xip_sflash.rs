use crate::rom_lookup::{rom_lookup, RomIndex};
use crate::rom::BL_Err_Type;
use crate::rom::SPI_Flash_Cfg_Type;

#[inline(always)]
pub fn XIP_SFlash_State_Save(pFlashCfg: *mut SPI_Flash_Cfg_Type, offset: *mut u32) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, *mut u32) -> BL_Err_Type,
        >(rom_lookup(RomIndex::XIP_SFlash_State_Save))
    };
    romdriver_func(pFlashCfg, offset)
}

#[inline(always)]
pub fn XIP_SFlash_State_Restore(pFlashCfg: *mut SPI_Flash_Cfg_Type, offset: u32) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type, u32) -> BL_Err_Type>(
            rom_lookup(RomIndex::XIP_SFlash_State_Restore),
        )
    };
    romdriver_func(pFlashCfg, offset)
}

#[inline(always)]
pub fn XIP_SFlash_Erase_Need_Lock(
    pFlashCfg: *mut SPI_Flash_Cfg_Type,
    startaddr: u32,
    endaddr: u32,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, u32, u32) -> BL_Err_Type,
        >(rom_lookup(RomIndex::XIP_SFlash_Erase_Need_Lock))
    };
    romdriver_func(pFlashCfg, startaddr, endaddr)
}

#[inline(always)]
pub fn XIP_SFlash_Write_Need_Lock(
    pFlashCfg: *mut SPI_Flash_Cfg_Type,
    addr: u32,
    data: *mut u8,
    len: u32,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, u32, *mut u8, u32) -> BL_Err_Type,
        >(rom_lookup(RomIndex::XIP_SFlash_Write_Need_Lock))
    };
    romdriver_func(pFlashCfg, addr, data, len)
}

#[inline(always)]
pub fn XIP_SFlash_Read_Need_Lock(
    pFlashCfg: *mut SPI_Flash_Cfg_Type,
    addr: u32,
    data: *mut u8,
    len: u32,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, u32, *mut u8, u32) -> BL_Err_Type,
        >(rom_lookup(RomIndex::XIP_SFlash_Read_Need_Lock))
    };
    romdriver_func(pFlashCfg, addr, data, len)
}

#[inline(always)]
pub fn XIP_SFlash_GetJedecId_Need_Lock(
    pFlashCfg: *mut SPI_Flash_Cfg_Type,
    data: *mut u8,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, *mut u8) -> BL_Err_Type,
        >(rom_lookup(RomIndex::XIP_SFlash_GetJedecId_Need_Lock))
    };
    romdriver_func(pFlashCfg, data)
}

#[inline(always)]
pub fn XIP_SFlash_GetDeviceId_Need_Lock(
    pFlashCfg: *mut SPI_Flash_Cfg_Type,
    data: *mut u8,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, *mut u8) -> BL_Err_Type,
        >(rom_lookup(RomIndex::XIP_SFlash_GetDeviceId_Need_Lock))
    };
    romdriver_func(pFlashCfg, data)
}

#[inline(always)]
pub fn XIP_SFlash_GetUniqueId_Need_Lock(
    pFlashCfg: *mut SPI_Flash_Cfg_Type,
    data: *mut u8,
    idLen: u8,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, *mut u8, u8) -> BL_Err_Type,
        >(rom_lookup(RomIndex::XIP_SFlash_GetUniqueId_Need_Lock))
    };
    romdriver_func(pFlashCfg, data, idLen)
}

#[inline(always)]
pub fn XIP_SFlash_Read_Via_Cache_Need_Lock(addr: u32, data: *mut u8, len: u32) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(u32, *mut u8, u32) -> BL_Err_Type>(
            rom_lookup(RomIndex::XIP_SFlash_Read_Via_Cache_Need_Lock),
        )
    };
    romdriver_func(addr, data, len)
}

#[inline(always)]
pub fn XIP_SFlash_Read_With_Lock(
    pFlashCfg: *mut SPI_Flash_Cfg_Type,
    addr: u32,
    dst: *mut u8,
    len: u32,
) -> u32 {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, u32, *mut u8, u32) -> BL_Err_Type,
        >(rom_lookup(RomIndex::XIP_SFlash_Read_With_Lock))
    };
    romdriver_func(pFlashCfg, addr, dst, len)
}

#[inline(always)]
pub fn XIP_SFlash_Write_With_Lock(
    pFlashCfg: *mut SPI_Flash_Cfg_Type,
    addr: u32,
    src: *mut u8,
    len: u32,
) -> u32 {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, u32, *mut u8, u32) -> BL_Err_Type,
        >(rom_lookup(RomIndex::XIP_SFlash_Write_With_Lock))
    };
    romdriver_func(pFlashCfg, addr, src, len)
}

#[inline(always)]
pub fn XIP_SFlash_Erase_With_Lock(pFlashCfg: *mut SPI_Flash_Cfg_Type, addr: u32, len: u32) -> u32 {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, u32, u32) -> BL_Err_Type,
        >(rom_lookup(RomIndex::XIP_SFlash_Erase_With_Lock))
    };
    romdriver_func(pFlashCfg, addr, len)
}

#[inline(always)]
pub fn XIP_SFlash_Opt_Enter(aesEnable: *mut u8) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut u8)>(rom_lookup(
            RomIndex::XIP_SFlash_Opt_Enter,
        ))
    };
    romdriver_func(aesEnable)
}

#[inline(always)]
pub fn XIP_SFlash_Opt_Exit(aesEnable: u8) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(u8)>(rom_lookup(
            RomIndex::XIP_SFlash_Opt_Exit,
        ))
    };
    romdriver_func(aesEnable)
}
