use crate::rom_lookup::{rom_lookup, RomIndex};
use crate::rom::BL_Err_Type;
use crate::rom::SPI_Flash_Cfg_Type;

/****************************************************************************//**
 * Save flash controller state
 *
 * `pFlashCfg` - Flash config pointer
 *
 * `offset` - CPU XIP flash offset pointer
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
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

/****************************************************************************//**
 * Restore flash controller state
 *
 * `pFlashCfg` - Flash config pointer
 *
 * `offset` - CPU XIP flash offset
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn XIP_SFlash_State_Restore(pFlashCfg: *mut SPI_Flash_Cfg_Type, offset: u32) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type, u32) -> BL_Err_Type>(
            rom_lookup(RomIndex::XIP_SFlash_State_Restore),
        )
    };
    romdriver_func(pFlashCfg, offset)
}

/****************************************************************************//**
 * Erase flash one region
 *
 * `pFlashCfg` - Flash config pointer
 *
 * `startaddr` - start address to erase
 *
 * `endaddr` - end address(include this address) to erase
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
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

/****************************************************************************//**
 * Program flash one region
 *
 * `pFlashCfg` - Flash config pointer
 *
 * `addr` - start address to be programed
 *
 * `data` - data pointer to be programed
 *
 * `len` - data length to be programed
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
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

/****************************************************************************//**
 * Read data from flash
 *
 * `pFlashCfg` - Flash config pointer
 *
 * `addr` - flash read start address
 *
 * `data` - data pointer to store data read from flash
 *
 * `len` - data length to read
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
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

/****************************************************************************//**
 * Get Flash Jedec ID
 *
 * `pFlashCfg` - Flash config pointer
 *
 * `data` - data pointer to store Jedec ID Read from flash
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
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

/****************************************************************************//**
 * Get Flash Device ID
 *
 * `pFlashCfg` - Flash config pointer
 *
 * `data` - data pointer to store Device ID Read from flash
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
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

/****************************************************************************//**
 * Get Flash Unique ID
 *
 * `pFlashCfg` - Flash config pointer
 *
 * `data` - data pointer to store Device ID Read from flash
 *
 * `idLen` - Unique id len
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
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

/****************************************************************************//**
 * Read data from flash via XIP
 *
 * `addr` - flash read start address
 *
 * `data` - data pointer to store data read from flash
 *
 * `len` - data length to read
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn XIP_SFlash_Read_Via_Cache_Need_Lock(addr: u32, data: *mut u8, len: u32) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(u32, *mut u8, u32) -> BL_Err_Type>(
            rom_lookup(RomIndex::XIP_SFlash_Read_Via_Cache_Need_Lock),
        )
    };
    romdriver_func(addr, data, len)
}


/****************************************************************************//**
 * Read data from flash with lock
 *
 * `pFlashCfg` - Flash config pointer
 *
 * `addr` - flash read start address
 *
 * `dst` - data pointer to store data read from flash
 *
 * `len` - data length to read
 *
 * Returns 0
 *
*******************************************************************************/
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

/****************************************************************************//**
 * Program flash one region with lock
 *
 * `pFlashCfg` - Flash config pointer
 *
 * `addr` - Start address to be programed
 *
 * `src` - Data pointer to be programed
 *
 * `len` - Data length to be programed
 *
 * Returns 0
 *
*******************************************************************************/
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

/****************************************************************************//**
 * Erase flash one region with lock
 *
 * `pFlashCfg` - Flash config pointer
 *
 * `addr` - Start address to be erased
 *
 * `len` - Data length to be erased
 *
 * Returns 0
 *
*******************************************************************************/
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

/****************************************************************************//**
 * XIP SFlash option save
 *
 * `aesEnable` - AES enable status pointer
 *
*******************************************************************************/
#[inline(always)]
pub fn XIP_SFlash_Opt_Enter(aesEnable: *mut u8) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut u8)>(rom_lookup(
            RomIndex::XIP_SFlash_Opt_Enter,
        ))
    };
    romdriver_func(aesEnable)
}

/****************************************************************************//**
 * XIP SFlash option restore
 *
 * `aesEnable` - AES enable status
 *
*******************************************************************************/
#[inline(always)]
pub fn XIP_SFlash_Opt_Exit(aesEnable: u8) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(u8)>(rom_lookup(
            RomIndex::XIP_SFlash_Opt_Exit,
        ))
    };
    romdriver_func(aesEnable)
}
