#![allow(clippy::upper_case_acronyms)]
use crate::rom::{
    BL_Err_Type, BL_Sts_Type, SF_Ctrl_Cfg_Type, SF_Ctrl_IO_Type, SF_Ctrl_Mode_Type,
    SPI_Flash_Cfg_Type,
};
use crate::rom_lookup::{rom_lookup, RomIndex};

/****************************************************************************/
/**
 * Init serial flash control interface
 *
 * `pSfCtrlCfg`: Serial flash controller configuration pointer
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Init(sfCtrlCfg: *const SF_Ctrl_Cfg_Type) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*const SF_Ctrl_Cfg_Type)>(rom_lookup(
            RomIndex::SFlash_Init,
        ))
    };
    romdriver_func(sfCtrlCfg)
}

/****************************************************************************/
/**
 * Set serial flash control interface SPI or QPI mode
 *
 * `mode` - Serial flash interface mode
 *
 * Returns BFLB_RET:SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_SetSPIMode(mode: SF_Ctrl_Mode_Type) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(SF_Ctrl_Mode_Type) -> u32>(rom_lookup(
            RomIndex::SFlash_SetSPIMode,
        ))
    };
    romdriver_func(mode)
}

/****************************************************************************/
/**
 * Read flash register
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
 * `regIndex` - register index
 *
 * `regValue` - register value pointer to store data
 *
 * `regLen` - register value length
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Read_Reg(
    flashCfg: *mut SPI_Flash_Cfg_Type,
    regIndex: u8,
    regValue: *mut u8,
    regLen: u8,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, u8, *mut u8, u8) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Read_Reg))
    };
    romdriver_func(flashCfg, regIndex, regValue, regLen)
}

/****************************************************************************/
/**
 * Write flash register
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
 * `regIndex` - register index
 *
 * `regValue` - register value pointer storing data
 *
 * `regLen` - register value length
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Write_Reg(
    flashCfg: *mut SPI_Flash_Cfg_Type,
    regIndex: u8,
    regValue: *mut u8,
    regLen: u8,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, u8, *mut u8, u8) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Write_Reg))
    };
    romdriver_func(flashCfg, regIndex, regValue, regLen)
}

/****************************************************************************/
/**
 * Check flash busy status
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
 * Returns SET for busy or RESET for not busy
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Busy(flashCfg: *mut SPI_Flash_Cfg_Type) -> BL_Sts_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type) -> BL_Sts_Type>(
            rom_lookup(RomIndex::SFlash_Busy),
        )
    };
    romdriver_func(flashCfg)
}

/****************************************************************************/
/**
 * Enable flash write function
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Write_Enable(flashCfg: *mut SPI_Flash_Cfg_Type) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type) -> BL_Err_Type>(
            rom_lookup(RomIndex::SFlash_Write_Enable),
        )
    };
    romdriver_func(flashCfg)
}

/****************************************************************************/
/**
 * Enable flash flash controller QSPI interface
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Qspi_Enable(flashCfg: *mut SPI_Flash_Cfg_Type) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type) -> BL_Err_Type>(
            rom_lookup(RomIndex::SFlash_Qspi_Enable),
        )
    };
    romdriver_func(flashCfg)
}

/****************************************************************************/
/**
 * Enable flash volatile register write enable
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Volatile_Reg_Write_Enable(flashCfg: *mut SPI_Flash_Cfg_Type) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type)>(rom_lookup(
            RomIndex::SFlash_Volatile_Reg_Write_Enable,
        ))
    };
    romdriver_func(flashCfg)
}

/****************************************************************************/
/**
 * Erase flash whole chip
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Chip_Erase(flashCfg: *mut SPI_Flash_Cfg_Type) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type) -> BL_Err_Type>(
            rom_lookup(RomIndex::SFlash_Chip_Erase),
        )
    };
    romdriver_func(flashCfg)
}

/****************************************************************************/
/**
 * Erase flash one sector
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 * `secNum` - flash sector number
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Sector_Erase(flashCfg: *mut SPI_Flash_Cfg_Type, secNum: u32) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type, u32) -> BL_Err_Type>(
            rom_lookup(RomIndex::SFlash_Sector_Erase),
        )
    };
    romdriver_func(flashCfg, secNum)
}

/****************************************************************************/
/**
 * Erase flash one 32K block
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
 * `blkNum` - flash 32K block number
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Blk32_Erase(flashCfg: *mut SPI_Flash_Cfg_Type, blkNum: u32) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type, u32) -> BL_Err_Type>(
            rom_lookup(RomIndex::SFlash_Blk32_Erase),
        )
    };
    romdriver_func(flashCfg, blkNum)
}

/****************************************************************************/
/**
 * Erase flash one 64K block
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
 * `blkNum` - flash 64K block number
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Blk64_Erase(flashCfg: *mut SPI_Flash_Cfg_Type, blkNum: u32) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type, u32) -> BL_Err_Type>(
            rom_lookup(RomIndex::SFlash_Blk64_Erase),
        )
    };
    romdriver_func(flashCfg, blkNum)
}

/****************************************************************************/
/**
 * Erase flash one region
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
 * `startaddr` - Start address to erase
 *
 * `endaddr` - end address(include this address) to erase
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Erase(
    flashCfg: *mut SPI_Flash_Cfg_Type,
    startaddr: u32,
    endaddr: u32,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, u32, u32) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Erase))
    };
    romdriver_func(flashCfg, startaddr, endaddr)
}

/****************************************************************************/
/**
 * Program flash one region
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
 * `ioMode: progran mode:SPI mode or QPI mode
 *
 * `addr` - Start address to be programed
 *
 * `data` - data pointer to be programed
 *
 * `len` - data length to be programed
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Program(
    flashCfg: *mut SPI_Flash_Cfg_Type,
    ioMode: SF_Ctrl_IO_Type,
    addr: u32,
    data: *mut u8,
    len: u32,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(
                *mut SPI_Flash_Cfg_Type,
                SF_Ctrl_IO_Type,
                u32,
                *mut u8,
                u32,
            ) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Program))
    };
    romdriver_func(flashCfg, ioMode, addr, data, len)
}

/****************************************************************************/
/**
 * Get flash unique ID
 *
 * `data` - data pointer to store read data
 *
 * `idLen` - unique ID len
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_GetUniqueId(data: *mut u8, idLen: u8) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut u8, u8)>(rom_lookup(
            RomIndex::SFlash_GetUniqueId,
        ))
    };
    romdriver_func(data, idLen)
}

/****************************************************************************/
/**
 * Get flash jedec ID
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
 * `data` - data pointer to store read data
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_GetJedecId(flashCfg: *mut SPI_Flash_Cfg_Type, data: *mut u8) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type, *mut u8)>(
            rom_lookup(RomIndex::SFlash_GetJedecId),
        )
    };
    romdriver_func(flashCfg, data)
}

/****************************************************************************/
/**
 * Get flash device ID
 *
 * `data` - data pointer to store read data
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_GetDeviceId(data: *mut u8) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut u8)>(rom_lookup(
            RomIndex::SFlash_GetDeviceId,
        ))
    };
    romdriver_func(data)
}

/****************************************************************************/
/**
 * Set flash power down
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Powerdown() {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn()>(rom_lookup(RomIndex::SFlash_Powerdown))
    };
    romdriver_func()
}

/****************************************************************************/
/**
 * Release flash power down for wake up
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Releae_Powerdown(flashCfg: *mut SPI_Flash_Cfg_Type) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type)>(rom_lookup(
            RomIndex::SFlash_Releae_Powerdown,
        ))
    };
    romdriver_func(flashCfg)
}

/****************************************************************************/
/**
 * Set flash burst wrap config
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_SetBurstWrap(flashCfg: *mut SPI_Flash_Cfg_Type) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type)>(rom_lookup(
            RomIndex::SFlash_SetBurstWrap,
        ))
    };
    romdriver_func(flashCfg)
}

/****************************************************************************/
/**
 * Disable flash burst wrap config
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_DisableBurstWrap(flashCfg: *mut SPI_Flash_Cfg_Type) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type)>(rom_lookup(
            RomIndex::SFlash_DisableBurstWrap,
        ))
    };
    romdriver_func(flashCfg)
}

/****************************************************************************/
/**
 * Software reset flash
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Software_Reset(flashCfg: *mut SPI_Flash_Cfg_Type) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type) -> BL_Err_Type>(
            rom_lookup(RomIndex::SFlash_Software_Reset),
        )
    };
    romdriver_func(flashCfg)
}

/****************************************************************************/
/**
 * Reset flash continous read mode
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Reset_Continue_Read(flashCfg: *mut SPI_Flash_Cfg_Type) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type)>(rom_lookup(
            RomIndex::SFlash_Reset_Continue_Read,
        ))
    };
    romdriver_func(flashCfg)
}

/****************************************************************************/
/**
 * Set I/D bus read flash configuration in flash controller
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
 * `ioMode` - flash controller interface mode
 *
 * `contRead` - Wether enable cont read mode
 *
 * `addr` - address to read/write
 *
 * `len` - data length to read/write
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Set_IDbus_Cfg(
    flashCfg: *mut SPI_Flash_Cfg_Type,
    ioMode: SF_Ctrl_IO_Type,
    contRead: u8,
    addr: u32,
    len: u32,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, SF_Ctrl_IO_Type, u8, u32, u32) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Set_IDbus_Cfg))
    };
    romdriver_func(flashCfg, ioMode, contRead, addr, len)
}

/****************************************************************************/
/**
 * Enable I/D bus read from flash
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
 * `ioMode` - flash controller interface mode
 *
 * `contRead` - Wether enable cont read mode
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_IDbus_Read_Enable(
    flashCfg: *mut SPI_Flash_Cfg_Type,
    ioMode: SF_Ctrl_IO_Type,
    contRead: u8,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, SF_Ctrl_IO_Type, u8) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_IDbus_Read_Enable))
    };
    romdriver_func(flashCfg, ioMode, contRead)
}

/****************************************************************************/
/**
 * Enable cache
 *
 * `wayDisable` - cache way disable config
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Cache_Enable_Set(wayDisable: u8) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(u8) -> BL_Err_Type>(rom_lookup(
            RomIndex::SFlash_Cache_Enable_Set,
        ))
    };
    romdriver_func(wayDisable)
}

/****************************************************************************/
/**
 * Flush cache
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Cache_Flush() -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn() -> BL_Err_Type>(rom_lookup(
            RomIndex::SFlash_Cache_Flush,
        ))
    };
    romdriver_func()
}

/****************************************************************************/
/**
 * Enable cache read from flash with cache
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
 * `ioMode` - flash controller interface mode
 *
 * `contRead` - Wether enable cont read mode
 *
 * `wayDisable` - cache way disable config
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Cache_Read_Enable(
    flashCfg: *mut SPI_Flash_Cfg_Type,
    ioMode: SF_Ctrl_IO_Type,
    contRead: u8,
    wayDisable: u8,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, SF_Ctrl_IO_Type, u8, u8) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Cache_Read_Enable))
    };
    romdriver_func(flashCfg, ioMode, contRead, wayDisable)
}

/****************************************************************************/
/**
 * Get cache hit count
 *
 * `hitCountLow` - hit count low 32 bits pointer
 *
 * `hitCountHigh` - hit count high 32 bits pointer
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Cache_Hit_Count_Get(hitCountLow: *mut u32, hitCountHigh: *mut u32) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut u32, *mut u32)>(rom_lookup(
            RomIndex::SFlash_Cache_Hit_Count_Get,
        ))
    };
    romdriver_func(hitCountLow, hitCountHigh)
}

/****************************************************************************/
/**
 * Get cache miss count
 *
 * Returns Cache miss count
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Cache_Miss_Count_Get() -> u32 {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn() -> u32>(rom_lookup(
            RomIndex::SFlash_Cache_Miss_Count_Get,
        ))
    };
    romdriver_func()
}

/****************************************************************************/
/**
 * Disable read from flash with cache
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Cache_Read_Disable() {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn()>(rom_lookup(
            RomIndex::SFlash_Cache_Read_Disable,
        ))
    };
    romdriver_func()
}

/****************************************************************************/
/**
 * Read data from flash
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
 * `ioMode` - flash controller interface mode
 *
 * `contRead` - Wether enable cont read mode
 *
 * `addr` - flash read start address
 *
 * `data` - data pointer to store data read from flash
 *
 * `len` - data length to read
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Read(
    flashCfg: *mut SPI_Flash_Cfg_Type,
    ioMode: SF_Ctrl_IO_Type,
    contRead: u8,
    addr: u32,
    data: *mut u8,
    len: u32,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(
                *mut SPI_Flash_Cfg_Type,
                SF_Ctrl_IO_Type,
                u8,
                u32,
                *mut u8,
                u32,
            ) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Read))
    };
    romdriver_func(flashCfg, ioMode, contRead, addr, data, len)
}

/****************************************************************************/
/**
 * Read flash register with read command
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
 * `readRegCmd` - read command
 *
 * `regValue` - register value pointer to store data
 *
 * `regLen` - register value length
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Read_Reg_With_Cmd(
    flashCfg: *mut SPI_Flash_Cfg_Type,
    readRegCmd: u8,
    regValue: *mut u8,
    regLen: u8,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, u8, *mut u8, u8) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Read_Reg_With_Cmd))
    };
    romdriver_func(flashCfg, readRegCmd, regValue, regLen)
}

/****************************************************************************/
/**
 * Write flash register with write command
 *
 * `flashCfg` - Serial flash parameter configuration pointer
 *
 * `writeRegCmd` - write command
 *
 * `regValue` - register value pointer storing data
 *
 * `regLen` - register value length
 *
 * Returns SUCCESS or ERROR
 *
*******************************************************************************/
#[inline(always)]
pub fn SFlash_Write_Reg_With_Cmd(
    flashCfg: *mut SPI_Flash_Cfg_Type,
    writeRegCmd: u8,
    regValue: *mut u8,
    regLen: u8,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, u8, *mut u8, u8) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Write_Reg_With_Cmd))
    };
    romdriver_func(flashCfg, writeRegCmd, regValue, regLen)
}
